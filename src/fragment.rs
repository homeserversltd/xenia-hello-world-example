use axum::{
    http::{header, HeaderValue},
    response::{Html, IntoResponse, Response},
};

pub async fn get() -> Response {
    let mut response = Html(xenia_kit::test_tab().into_string()).into_response();
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response.headers_mut().insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(xenia_kit::fragment::CROWN_CONTENT_SECURITY_POLICY),
    );
    response
}
