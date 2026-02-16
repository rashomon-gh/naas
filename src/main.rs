use axum::{
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::any,
    Router,
};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize)]
struct NothingResponse {
    result: String,
}

async fn fallback_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(NothingResponse {
            result: String::from("nothing"),
        }),
    )
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", any(fallback_handler))
        .route("/{*path}", any(fallback_handler))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    println!("Nothing as a Service listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.expect("Server error");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{header, Method, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_root_path_returns_nothing() {
        let app = create_test_app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["result"], "nothing");
    }

    #[tokio::test]
    async fn test_arbitrary_path_returns_nothing() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/some/arbitrary/path")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["result"], "nothing");
    }

    #[tokio::test]
    async fn test_deeply_nested_path_returns_nothing() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/users/123/posts/456/comments")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["result"], "nothing");
    }

    #[tokio::test]
    async fn test_cors_headers_present() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/test")
                    .header(header::ORIGIN, "https://example.com")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(response
            .headers()
            .contains_key("access-control-allow-origin"));
    }

    #[tokio::test]
    async fn test_get_request_returns_nothing() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/anything")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["result"], "nothing");
    }

    #[tokio::test]
    async fn test_post_request_returns_nothing() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/submit")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["result"], "nothing");
    }

    fn create_test_app() -> Router {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        Router::new()
            .route("/", any(fallback_handler))
            .route("/{*path}", any(fallback_handler))
            .layer(cors)
    }
}
