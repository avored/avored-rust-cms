use axum::{
    extract::connect_info::{ConnectInfo, Connected}, 
    http::header::CONTENT_TYPE, 
    Router
};
use futures::ready;
use hyper::{Request, Response};
use std::{
    convert::Infallible,
    task::{Context, Poll},
    any::Any,
    net::SocketAddr,
};
use tonic::transport::server::TcpConnectInfo;
use tower::{make::Shared, Service};

/// This service splits all incoming requests either to the rest-service, or to
/// the grpc-service based on the `content-type` header.
///
/// Only if the header `content-type = application/grpc` exists, will the requests be handled
/// by the grpc-service. All other requests go to the rest-service.
#[derive(Debug, Clone)]
pub struct RestGrpcService {
    rest_router: Router,
    rest_ready: bool,
    grpc_router: Router,
    grpc_ready: bool,
}

impl RestGrpcService {
    /// Create a new service, which splits requests between the rest- and grpc-router.
    pub fn new(rest_router: Router, grpc_router: Router) -> Self {
        Self {
            rest_router,
            rest_ready: false,
            grpc_router,
            grpc_ready: false,
        }
    }

    /// Create a make-service from this service. This make-service can be directly used
    /// in the `serve` method of an axum/hyper Server.
    ///
    /// If you would like to add shared middleware for both the rest-service and the grpc-service,
    /// the following approach is recommended:
    ///
    /// ```ignore
    /// use axum_tonic::RestGrpcService;
    /// use tokio::net::TcpListener;
    /// use tower::ServiceBuilder;
    ///
    /// let svc: RestGrpcService = my_service();
    ///
    /// let svc_with_layers = ServiceBuilder::new()
    ///     .buffer(5)
    ///     .layer(my_layer1())
    ///     .layer(my_layer2())
    ///     .service(svc);
    ///
    /// axum::serve(TcpListener::bind(&"127.0.0.1:3000"), svc_with_layers)
    ///     .await
    ///     .unwrap();
    /// ```
    pub fn into_make_service(self) -> Shared<Self> {
        Shared::new(self)
    }

    /// Create a make-service with connect info from this service.
    /// This allows you to extract connection information in your handlers using
    /// `extract::ConnectInfo<C>`.
    ///
    /// Example:
    ///
    /// ```ignore
    /// use axum_tonic::RestGrpcService;
    /// use axum::extract::ConnectInfo;
    /// use tokio::net::TcpListener;
    /// use std::net::SocketAddr;
    ///
    /// // Create a router with a handler that uses connect info
    /// let rest_router = axum::Router::new()
    ///     .route("/", axum::routing::get(
    ///         |ConnectInfo(addr): ConnectInfo<SocketAddr>| async move {
    ///             format!("Hello from IP: {}", addr)
    ///         }
    ///     ));
    ///
    /// let grpc_router = // Your gRPC router
    ///
    /// let svc = RestGrpcService::new(rest_router, grpc_router);
    ///
    /// // Use with connect info
    /// axum::serve(
    ///     TcpListener::bind("127.0.0.1:3000").await.unwrap(),
    ///     svc.into_make_service_with_connect_info::<SocketAddr>()
    /// ).await.unwrap();
    /// ```
    pub fn into_make_service_with_connect_info<C>(self) -> RestGrpcMakeServiceWithConnectInfo<C> 
    where
        C: Send + Sync + Clone + 'static,
    {
        RestGrpcMakeServiceWithConnectInfo {
            inner: self,
            _connect_info: std::marker::PhantomData,
        }
    }
}


/// A wrapper service that captures connection info and passes it to the inner service.
#[derive(Clone)]
pub struct RestGrpcMakeServiceWithConnectInfo<C> {
    inner: RestGrpcService,
    _connect_info: std::marker::PhantomData<fn() -> C>,
}

impl<C, Target> Service<Target> for RestGrpcMakeServiceWithConnectInfo<C>
where
    C: Connected<Target> + Send + Sync + Clone + 'static,
    Target: Send, // Target is typically &'a AddrStream, which is Send but not 'static
{
    type Response = RestGrpcServiceWithConnectInfo<C>;
    type Error = Infallible;
    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        let connect_info = C::connect_info(target);
        let inner = self.inner.clone();

        std::future::ready(Ok(RestGrpcServiceWithConnectInfo {
            inner,
            connect_info,
        }))
    }
}

/// A service that holds both the RestGrpcService and the connection info.
#[derive(Clone)]
pub struct RestGrpcServiceWithConnectInfo<C> {
    inner: RestGrpcService,
    connect_info: C,
}

impl<C, ReqBody> Service<Request<ReqBody>> for RestGrpcServiceWithConnectInfo<C>
where
    C: Send + Sync + Clone + 'static,
    ReqBody: http_body::Body<Data = axum::body::Bytes> + Send + 'static,
    ReqBody::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    type Response = Response<axum::body::Body>;
    type Error = Infallible;
    type Future = <Router as Service<Request<ReqBody>>>::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // drive readiness for each inner service and record which is ready
        loop {
            match (self.inner.rest_ready, self.inner.grpc_ready) {
                (true, true) => {
                    return Ok(()).into();
                }
                (false, _) => {
                    ready!(
                        <axum::Router as tower::Service<Request<ReqBody>>>::poll_ready(
                            &mut self.inner.rest_router,
                            cx
                        )
                    )?;
                    self.inner.rest_ready = true;
                }
                (_, false) => {
                    ready!(
                        <axum::Router as tower::Service<Request<ReqBody>>>::poll_ready(
                            &mut self.inner.rest_router,
                            cx
                        )
                    )?;
                    self.inner.grpc_ready = true;
                }
            }
        }
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        // require users to call `poll_ready` first, if they don't we're allowed to panic
        // as per the `tower::Service` contract
        assert!(
            self.inner.grpc_ready,
            "grpc service not ready. Did you forget to call `poll_ready`?"
        );
        assert!(
            self.inner.rest_ready,
            "rest service not ready. Did you forget to call `poll_ready`?"
        );

        // Store connect info in request extensions for all requests passing through
        // this service, so Axum handlers in both rest_router and grpc_router can access it.
        req.extensions_mut().insert(ConnectInfo(self.connect_info.clone()));

        // tonic via grpc_router should also have a TcpConnectInfo, this will populated
        // `request.remote_addr()` but not `.local_addr()`.
        if let Some(socket_addr) = (&self.connect_info as &dyn Any).downcast_ref::<SocketAddr>() {
            req.extensions_mut().insert(TcpConnectInfo {
                local_addr: None,
                remote_addr: Some(*socket_addr),
            });
        }

        // if we get a grpc request call the grpc service, otherwise call the rest service
        // when calling a service it becomes not-ready so we have drive readiness again
        if is_grpc_request(&req) {
            self.inner.grpc_ready = false;
            self.inner.grpc_router.call(req)
        } else {
            self.inner.rest_ready = false;
            self.inner.rest_router.call(req)
        }
    }
}

impl<ReqBody> Service<Request<ReqBody>> for RestGrpcService
where
    ReqBody: http_body::Body<Data = axum::body::Bytes> + Send + 'static,
    ReqBody::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    type Response = Response<axum::body::Body>;
    type Error = Infallible;
    type Future = <Router as Service<Request<ReqBody>>>::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // drive readiness for each inner service and record which is ready
        loop {
            match (self.rest_ready, self.grpc_ready) {
                (true, true) => {
                    return Ok(()).into();
                }
                (false, _) => {
                    ready!(
                        <axum::Router as tower::Service<Request<ReqBody>>>::poll_ready(
                            &mut self.rest_router,
                            cx
                        )
                    )?;
                    self.rest_ready = true;
                }
                (_, false) => {
                    ready!(
                        <axum::Router as tower::Service<Request<ReqBody>>>::poll_ready(
                            &mut self.rest_router,
                            cx
                        )
                    )?;
                    self.grpc_ready = true;
                }
            }
        }
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        // require users to call `poll_ready` first, if they don't we're allowed to panic
        // as per the `tower::Service` contract
        assert!(
            self.grpc_ready,
            "grpc service not ready. Did you forget to call `poll_ready`?"
        );
        assert!(
            self.rest_ready,
            "rest service not ready. Did you forget to call `poll_ready`?"
        );

        // if we get a grpc request call the grpc service, otherwise call the rest service
        // when calling a service it becomes not-ready so we have drive readiness again
        if is_grpc_request(&req) {
            self.grpc_ready = false;
            self.grpc_router.call(req)
        } else {
            self.rest_ready = false;
            self.rest_router.call(req)
        }
    }
}

fn is_grpc_request<B>(req: &Request<B>) -> bool {
    req.headers()
        .get(CONTENT_TYPE)
        .map(|content_type| content_type.as_bytes())
        .filter(|content_type| content_type.starts_with(b"application/grpc"))
        .is_some()
}