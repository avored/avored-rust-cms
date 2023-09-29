use std::{
    task::{Context, Poll},
    time::Duration,
};

use async_session::{
    base64,
    hmac::{Hmac, Mac, NewMac},
    sha2::Sha256,
    SessionStore,
};
use axum::{
    http::{
        header::{HeaderValue, COOKIE, SET_COOKIE},
        Request, StatusCode,
    },
    response::Response,
};
use axum_extra::extract::cookie::{Cookie, Key, SameSite};
use futures::future::BoxFuture;
use tower::{Layer, Service};
use tracing::error;

use std::ops::{Deref, DerefMut};

use axum::{async_trait, extract::FromRequestParts, http::request::Parts, Extension};

const BASE64_DIGEST_LEN: usize = 44;

pub type SessionHandle = async_session::Session;

// #[derive(Clone)]
// pub enum PersistencePolicy {
//     Always,
//     ChangedOnly,
//     ExistingOnly,
// }

/// Layer that provides cookie-based sessions.
#[derive(Clone)]
pub struct SessionLayer<Store> {
    store: Store,
    cookie_path: String,
    cookie_name: String,
    cookie_domain: Option<String>,
    session_ttl: Option<Duration>,
    same_site_policy: SameSite,
    http_only: bool,
    secure: bool,
    key: Key,
}

impl<Store: SessionStore> SessionLayer<Store> {
    /// Creates a layer which will attach a [`SessionHandle`] to requests via an
    /// extension. This session is derived from a cryptographically signed
    /// cookie. When the client sends a valid, known cookie then the session is
    /// hydrated from this. Otherwise a new cookie is created and returned in
    /// the response.
    ///
    /// The default behaviour is to enable "guest" sessions with
    /// [`PersistencePolicy::Always`].
    ///
    /// # Panics
    ///
    /// `SessionLayer::new` will panic if the secret is less than 64 bytes.
    ///
    /// # Customization
    ///
    /// The configuration of the session may be adjusted according to the needs
    /// of your application:
    ///
    /// ```rust
    /// # use axum_sessions::{PersistencePolicy, SessionLayer, async_session::MemoryStore, SameSite};
    /// # use std::time::Duration;
    /// SessionLayer::new(
    ///     MemoryStore::new(),
    ///     b"please do not hardcode your secret; instead use a
    ///     cryptographically secure value",
    /// )
    /// .with_cookie_name("your.cookie.name")
    /// .with_cookie_path("/some/path")
    /// .with_cookie_domain("www.example.com")
    /// .with_same_site_policy(SameSite::Lax)
    /// .with_session_ttl(Some(Duration::from_secs(60 * 5)))
    /// .with_persistence_policy(PersistencePolicy::Always)
    /// .with_http_only(true)
    /// .with_secure(true);
    /// ```
    pub fn new(store: Store, secret: &[u8]) -> Self {
        if secret.len() < 64 {
            panic!("`secret` must be at least 64 bytes.")
        }

        Self {
            store,
            // persistence_policy: PersistencePolicy::Always,
            cookie_path: "/".into(),
            cookie_name: "sid".into(),
            cookie_domain: None,
            same_site_policy: SameSite::Strict,
            session_ttl: Some(Duration::from_secs(24 * 60 * 60)),
            http_only: true,
            secure: true,
            key: Key::from(secret),
        }
    }

    // pub fn with_persistence_policy(mut self, policy: PersistencePolicy) -> Self {
    //     self.persistence_policy = policy;
    //     self
    // }

    // /// Sets a cookie for the session. Defaults to `"/"`.
    // pub fn with_cookie_path(mut self, cookie_path: impl AsRef<str>) -> Self {
    //     self.cookie_path = cookie_path.as_ref().to_owned();
    //     self
    // }

    // /// Sets a cookie name for the session. Defaults to `"sid"`.
    // pub fn with_cookie_name(mut self, cookie_name: impl AsRef<str>) -> Self {
    //     self.cookie_name = cookie_name.as_ref().to_owned();
    //     self
    // }

    // /// Sets a cookie domain for the session. Defaults to `None`.
    // pub fn with_cookie_domain(mut self, cookie_domain: impl AsRef<str>) -> Self {
    //     self.cookie_domain = Some(cookie_domain.as_ref().to_owned());
    //     self
    // }

    /// Decide if session is presented to the storage layer
    // fn should_store(&self, cookie_value: &Option<String>, session_data_changed: bool) -> bool {
    //     session_data_changed
    //         || matches!(self.persistence_policy, PersistencePolicy::Always)
    //         || (matches!(self.persistence_policy, PersistencePolicy::ExistingOnly)
    //             && cookie_value.is_some())
    // }

    // /// Sets a cookie same site policy for the session. Defaults to
    // /// `SameSite::Strict`.
    // pub fn with_same_site_policy(mut self, policy: SameSite) -> Self {
    //     self.same_site_policy = policy;
    //     self
    // }

    // /// Sets a cookie time-to-live (ttl) for the session. Defaults to
    // /// `Duration::from_secs(60 * 60 * 24)`; one day.
    // pub fn with_session_ttl(mut self, session_ttl: Option<Duration>) -> Self {
    //     self.session_ttl = session_ttl;
    //     self
    // }

    // /// Sets a cookie `HttpOnly` attribute for the session. Defaults to `true`.
    // pub fn with_http_only(mut self, http_only: bool) -> Self {
    //     self.http_only = http_only;
    //     self
    // }

    // /// Sets a cookie secure attribute for the session. Defaults to `true`.
    // pub fn with_secure(mut self, secure: bool) -> Self {
    //     self.secure = secure;
    //     self
    // }

    async fn load_or_create(&self, cookie_value: Option<String>) -> SessionHandle {
        let session = match cookie_value {
            Some(cookie_value) => self.store.load_session(cookie_value).await.ok().flatten(),
            None => None,
        };

        session
            .and_then(async_session::Session::validate)
            .unwrap_or_default()
    }

    fn build_cookie(&self, cookie_value: String) -> Cookie<'static> {
        let mut cookie = Cookie::build(self.cookie_name.clone(), cookie_value)
            .http_only(self.http_only)
            .same_site(self.same_site_policy)
            .secure(self.secure)
            .path(self.cookie_path.clone())
            .finish();

        if let Some(ttl) = self.session_ttl {
            cookie.set_expires(Some((std::time::SystemTime::now() + ttl).into()));
        }

        if let Some(cookie_domain) = self.cookie_domain.clone() {
            cookie.set_domain(cookie_domain)
        }

        self.sign_cookie(&mut cookie);

        cookie
    }

    fn build_removal_cookie(&self) -> Cookie<'static> {
        let cookie = Cookie::build(self.cookie_name.clone(), "")
            .http_only(true)
            .path(self.cookie_path.clone());

        let mut cookie = if let Some(cookie_domain) = self.cookie_domain.clone() {
            cookie.domain(cookie_domain)
        } else {
            cookie
        }
        .finish();

        cookie.make_removal();

        self.sign_cookie(&mut cookie);

        cookie
    }
    fn sign_cookie(&self, cookie: &mut Cookie<'_>) {
        // Compute HMAC-SHA256 of the cookie's value.
        let mut mac = Hmac::<Sha256>::new_from_slice(self.key.signing()).expect("good key");
        mac.update(cookie.value().as_bytes());

        // Cookie's new value is [MAC | original-value].
        let mut new_value = base64::encode(mac.finalize().into_bytes());
        new_value.push_str(cookie.value());
        cookie.set_value(new_value);
    }

    fn verify_signature(&self, cookie_value: &str) -> Result<String, &'static str> {
        if cookie_value.len() < BASE64_DIGEST_LEN {
            return Err("length of value is <= BASE64_DIGEST_LEN");
        }

        // Split [MAC | original-value] into its two parts.
        let (digest_str, value) = cookie_value.split_at(BASE64_DIGEST_LEN);
        let digest = base64::decode(digest_str).map_err(|_| "bad base64 digest")?;

        // Perform the verification.
        let mut mac = Hmac::<Sha256>::new_from_slice(self.key.signing()).expect("good key");
        mac.update(value.as_bytes());
        mac.verify(&digest)
            .map(|_| value.to_string())
            .map_err(|_| "value did not verify")
    }
}

impl<Inner, Store: SessionStore> Layer<Inner> for SessionLayer<Store> {
    type Service = Session<Inner, Store>;

    fn layer(&self, inner: Inner) -> Self::Service {
        Session {
            inner,
            layer: self.clone(),
        }
    }
}

/// Session service container.
#[derive(Clone)]
pub struct Session<Inner, Store: SessionStore> {
    inner: Inner,
    layer: SessionLayer<Store>,
}

impl<Inner, ReqBody, ResBody, Store: SessionStore> Service<Request<ReqBody>>
    for Session<Inner, Store>
where
    Inner: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    ResBody: Send + 'static,
    ReqBody: Send + 'static,
    Inner::Future: Send + 'static,
{
    type Response = Inner::Response;
    type Error = Inner::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<ReqBody>) -> Self::Future {
        let session_layer = self.layer.clone();

        // Multiple cookies may be all concatenated into a single Cookie header
        // separated with semicolons (HTTP/1.1 behaviour) or into multiple separate
        // Cookie headers (HTTP/2 behaviour). Search for the session cookie from
        // all Cookie headers, assuming both forms are possible
        let cookie_value = request
            .headers()
            .get_all(COOKIE)
            .iter()
            .filter_map(|cookie_header| cookie_header.to_str().ok())
            .flat_map(|cookie_header| cookie_header.split(';'))
            .filter_map(|cookie_header| Cookie::parse_encoded(cookie_header.trim()).ok())
            .filter(|cookie| cookie.name() == session_layer.cookie_name)
            .find_map(|cookie| self.layer.verify_signature(cookie.value()).ok());

        let inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, inner);
        Box::pin(async move {
            let session_handle = session_layer.load_or_create(cookie_value.clone()).await;

            let mut session = session_handle;
            if let Some(ttl) = session_layer.session_ttl {
                (session).expire_in(ttl);
            }
            request.extensions_mut().insert(session.clone());
            let mut response = inner.call(request).await?;

            let session: async_session::Session = session;
            let session_is_destroyed = session.is_destroyed();

            if session_is_destroyed {
                if let Err(e) = session_layer.store.destroy_session(session).await {
                    error!("Failed to destroy session: {:?}", e);
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                }

                let removal_cookie = session_layer.build_removal_cookie();

                response.headers_mut().append(
                    SET_COOKIE,
                    HeaderValue::from_str(&removal_cookie.to_string()).unwrap(),
                );
            } else {
                match session_layer.store.store_session(session).await {
                    Ok(Some(cookie_value)) => {
                        let cookie = session_layer.build_cookie(cookie_value);
                        response.headers_mut().append(
                            SET_COOKIE,
                            HeaderValue::from_str(&cookie.to_string()).unwrap(),
                        );
                    }

                    Ok(None) => {}

                    Err(e) => {
                        error!("Failed to reach session storage: {:?}", e);
                        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    }
                }
            }

            Ok(response)
        })
    }
}

#[derive(Debug, Clone)]
pub struct AvoRedSession {
    session: async_session::Session,
}

impl Deref for AvoRedSession {
    type Target = async_session::Session;

    fn deref(&self) -> &Self::Target {
        &self.session
    }
}

impl DerefMut for AvoRedSession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.session
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AvoRedSession
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(session_handle): Extension<SessionHandle> =
            Extension::from_request_parts(parts, state)
                .await
                .expect("Session extension missing. Is the session layer installed?");
        let session = session_handle.clone();

        Ok(Self { session })
    }
}
