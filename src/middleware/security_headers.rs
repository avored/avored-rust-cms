use axum::body::Body;
use axum::http::{header, HeaderValue, Request, Response};
use axum::middleware::Next;
use axum::response::IntoResponse;

/// Middleware to add security headers to all responses
pub async fn add_security_headers(
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, Response<Body>> {
    // Check if this is an API request before consuming the request
    let is_api_request = req.uri().path().starts_with("/api/");

    let mut response = next.run(req).await;

    let headers = response.headers_mut();

    // X-Content-Type-Options: Prevents MIME type sniffing
    headers.insert(
        header::HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    );

    // X-Frame-Options: Prevents clickjacking attacks
    headers.insert(
        header::HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY"),
    );

    // X-XSS-Protection: Enables XSS filtering (legacy browsers)
    headers.insert(
        header::HeaderName::from_static("x-xss-protection"),
        HeaderValue::from_static("1; mode=block"),
    );

    // Strict-Transport-Security: Enforces HTTPS
    headers.insert(
        header::HeaderName::from_static("strict-transport-security"),
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );

    // Content-Security-Policy: Prevents XSS and data injection attacks
    // Note: This is a basic policy, adjust based on your application's needs
    headers.insert(
        header::HeaderName::from_static("content-security-policy"),
        HeaderValue::from_static("default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data:; connect-src 'self'; frame-ancestors 'none';"),
    );

    // Referrer-Policy: Controls how much referrer information is included with requests
    headers.insert(
        header::HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );

    // Permissions-Policy: Controls which browser features can be used
    headers.insert(
        header::HeaderName::from_static("permissions-policy"),
        HeaderValue::from_static("camera=(), microphone=(), geolocation=(), payment=()"),
    );

    // X-Permitted-Cross-Domain-Policies: Restricts Adobe Flash and PDF cross-domain requests
    headers.insert(
        header::HeaderName::from_static("x-permitted-cross-domain-policies"),
        HeaderValue::from_static("none"),
    );

    // Cache-Control: Prevents caching of sensitive content (for API responses)
    if is_api_request {
        headers.insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-cache, no-store, must-revalidate"),
        );
        headers.insert(
            header::PRAGMA,
            HeaderValue::from_static("no-cache"),
        );
        headers.insert(
            header::EXPIRES,
            HeaderValue::from_static("0"),
        );
    }

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing::get, Router};
    use axum::http::StatusCode;
    use tower::util::ServiceExt;

    async fn test_handler() -> &'static str {
        "Hello, World!"
    }

    #[tokio::test]
    async fn test_security_headers_middleware() {
        let app = Router::new()
            .route("/", get(test_handler))
            .layer(axum::middleware::from_fn(add_security_headers));

        let request = Request::builder()
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let headers = response.headers();
        
        assert_eq!(
            headers.get("x-content-type-options").unwrap(),
            "nosniff"
        );
        assert_eq!(
            headers.get("x-frame-options").unwrap(),
            "DENY"
        );
        assert_eq!(
            headers.get("x-xss-protection").unwrap(),
            "1; mode=block"
        );
        assert_eq!(
            headers.get("strict-transport-security").unwrap(),
            "max-age=31536000; includeSubDomains"
        );
        assert!(headers.get("content-security-policy").is_some());
        assert_eq!(
            headers.get("referrer-policy").unwrap(),
            "strict-origin-when-cross-origin"
        );
    }

    #[tokio::test]
    async fn test_api_cache_headers() {
        let app = Router::new()
            .route("/api/test", get(test_handler))
            .layer(axum::middleware::from_fn(add_security_headers));

        let request = Request::builder()
            .uri("/api/test")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        let headers = response.headers();
        
        assert_eq!(
            headers.get("cache-control").unwrap(),
            "no-cache, no-store, must-revalidate"
        );
        assert_eq!(
            headers.get("pragma").unwrap(),
            "no-cache"
        );
        assert_eq!(
            headers.get("expires").unwrap(),
            "0"
        );
    }
}
