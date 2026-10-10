//! Request / correlation id propagation.
//!
//! An inbound `X-Request-Id` is accepted when it is short, printable ASCII; otherwise a UUIDv7 is
//! generated. The id is stored in request extensions, echoed on the response and made available
//! to error rendering through a task-local so every problem document carries `correlationId`.

use axum::extract::Request;
use axum::http::{HeaderName, HeaderValue};
use axum::middleware::Next;
use axum::response::Response;

pub static X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

tokio::task_local! {
    static CORRELATION_ID: String;
}

/// Request id stored in request extensions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestId(pub String);

/// Correlation id of the request currently being served, if any.
pub fn current_correlation_id() -> Option<String> {
    CORRELATION_ID.try_with(Clone::clone).ok()
}

fn acceptable(v: &str) -> bool {
    !v.is_empty() && v.len() <= 128 && v.bytes().all(|b| b.is_ascii_graphic())
}

pub async fn request_id(mut req: Request, next: Next) -> Response {
    let id = req
        .headers()
        .get(&X_REQUEST_ID)
        .and_then(|v| v.to_str().ok())
        .filter(|v| acceptable(v))
        .map(str::to_owned)
        .unwrap_or_else(|| sutra_kernel::id::new_v7().to_string());
    req.extensions_mut().insert(RequestId(id.clone()));
    let mut res = CORRELATION_ID.scope(id.clone(), next.run(req)).await;
    if let Ok(v) = HeaderValue::from_str(&id) {
        res.headers_mut().insert(X_REQUEST_ID.clone(), v);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request as HttpRequest, routing::get, Router};
    use tower::ServiceExt;

    fn app() -> Router {
        Router::new()
            .route(
                "/",
                get(|| async { current_correlation_id().unwrap_or_default() }),
            )
            .layer(axum::middleware::from_fn(request_id))
    }

    #[tokio::test]
    async fn propagates_inbound_id() {
        let res = app()
            .oneshot(
                HttpRequest::get("/")
                    .header("x-request-id", "abc-123")
                    .body(Body::empty())
                    .unwrap_or_default(),
            )
            .await
            .expect("response");
        assert_eq!(res.headers()["x-request-id"], "abc-123");
    }

    #[tokio::test]
    async fn generates_when_missing_or_invalid() {
        let res = app()
            .oneshot(
                HttpRequest::get("/")
                    .header("x-request-id", "bad id with spaces")
                    .body(Body::empty())
                    .unwrap_or_default(),
            )
            .await
            .expect("response");
        let id = res.headers()["x-request-id"]
            .to_str()
            .unwrap_or_default()
            .to_owned();
        assert!(uuid::Uuid::parse_str(&id).is_ok());
    }
}
