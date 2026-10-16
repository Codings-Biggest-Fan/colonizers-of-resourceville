//! Process liveness only; no rooms, identity, persistence, or game routes yet.
#![forbid(unsafe_code)]

pub fn app() -> axum::Router {
    axum::Router::new().route("/health", axum::routing::get(|| async { "ok\n" }))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_is_liveness_and_game_routes_are_absent() {
        let response = super::app()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            to_bytes(response.into_body(), 16).await.unwrap().as_ref(),
            b"ok\n"
        );
        let response = super::app()
            .oneshot(
                Request::builder()
                    .uri("/rooms")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
