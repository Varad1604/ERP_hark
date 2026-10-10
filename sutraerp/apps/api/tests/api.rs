//! Router-level tests of the assembled application. No database required.

#![allow(clippy::unwrap_used)]

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use http_body_util::BodyExt;
use sutra_api::app::{router, AppContext};
use tower::ServiceExt;

async fn call(req: Request<Body>) -> (StatusCode, axum::http::HeaderMap, serde_json::Value) {
    let res = router(AppContext::for_spec()).oneshot(req).await.unwrap();
    let status = res.status();
    let headers = res.headers().clone();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        headers,
        serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null),
    )
}

#[tokio::test]
async fn liveness_and_request_id() {
    let (s, h, body) = call(
        Request::get("/health/live")
            .header("x-request-id", "test-1")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(h["x-request-id"], "test-1");
    assert_eq!(body["status"], "ok");
}

#[tokio::test]
async fn readiness_fails_without_database() {
    let (s, _, body) = call(Request::get("/health/ready").body(Body::empty()).unwrap()).await;
    assert_eq!(s, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body["checks"][0]["name"], "postgres");
}

#[tokio::test]
async fn unknown_route_is_problem_json() {
    let (s, h, body) = call(
        Request::get("/api/v1/nope")
            .header("x-request-id", "rid-9")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    assert_eq!(h[header::CONTENT_TYPE], "application/problem+json");
    assert_eq!(body["correlationId"], "rid-9");
}

#[tokio::test]
async fn protected_routes_fail_closed_without_tenant() {
    let (s, _, body) = call(
        Request::get("/api/v1/institution/institutions/0192f5a0-7c3e-7000-8000-0000000000aa")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(s, StatusCode::BAD_REQUEST);
    assert_eq!(body["errorCode"], "scope-violation");
}

#[tokio::test]
async fn create_requires_idempotency_key() {
    let req = Request::post("/api/v1/institution/institutions")
        .header("content-type", "application/json")
        .header("x-sutra-tenant-id", "0192f5a0-7c3e-7000-8000-000000000001")
        .body(Body::from(r#"{"code":"SIT","name":"Sutra"}"#))
        .unwrap();
    let (s, _, body) = call(req).await;
    assert_eq!(s, StatusCode::BAD_REQUEST);
    assert!(body["detail"].as_str().unwrap().contains("Idempotency-Key"));
}

#[tokio::test]
async fn dev_login_sets_session_cookie() {
    let req = Request::post("/api/v1/auth/login")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"loginId":"meera.shah@sutra.dev","password":"sutra"}"#,
        ))
        .unwrap();
    let (s, h, body) = call(req).await;
    assert_eq!(s, StatusCode::OK);
    assert!(h[header::SET_COOKIE].to_str().unwrap().contains("HttpOnly"));
    assert_eq!(body["principal"]["roles"][0], "faculty");
}

#[tokio::test]
async fn catalog_and_modules() {
    let (s, _, body) = call(
        Request::get("/api/v1/platform/workspaces?role=faculty")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(body["total"], 50);
    let (_, _, modules) = call(
        Request::get("/api/v1/platform/modules")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(modules.as_array().map(Vec::len), Some(10));
}

#[tokio::test]
async fn openapi_is_served_and_matches_committed_spec() {
    let (s, _, body) = call(
        Request::get("/api/openapi.json")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(body["openapi"], "3.1.0");
    for p in [
        "/api/v1/auth/login",
        "/api/v1/platform/workspaces",
        "/health/ready",
    ] {
        assert!(body["paths"].get(p).is_some(), "missing {p}");
    }
    let committed = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../openapi/openapi.json"
    ))
    .unwrap();
    let generated = sutra_api::openapi::spec_json().unwrap();
    assert!(
        committed == generated,
        "openapi/openapi.json is stale: run `make openapi`"
    );
}
