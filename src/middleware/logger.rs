use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;

pub async fn logger_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    let response = next.run(req).await;

    let duration = start.elapsed();
    let status = response.status();

    let duration_ms = duration.as_secs_f64() * 1000.0;

    match status.as_u16() {
        200..=299 => {
            tracing::info!("{} {} {} {:.3}ms", method, uri.path(), status.as_u16(), duration_ms);
        },
        300..=399 => {
            tracing::info!("{} {} {} {:.3}ms", method, uri.path(), status.as_u16(), duration_ms);
        },
        400..=499 => {
            tracing::warn!("{} {} {} {:.3}ms", method, uri.path(), status.as_u16(), duration_ms);
        },
        500..=599 => {
            tracing::error!("{} {} {} {:.3}ms", method, uri.path(), status.as_u16(), duration_ms);
        },
        _ => {
            tracing::info!("{} {} {} {:.3}ms", method, uri.path(), status.as_u16(), duration_ms);
        },
    }

    response
}
