use axum::{handler::get, Router, Server};

use routes::*;

use std::future::Future;
use std::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::Level;

mod domain;
pub mod routes;
pub mod telemetry;
pub mod tracelog;

pub fn run(listener: TcpListener) -> Result<impl Future<Output = hyper::Result<()>>, hyper::Error> {
    let logger = tracelog::TracingLogger {
        req_level: Some(Level::INFO),
        _chunk_level: None,
        _eos_level: None,
        resp_level: Some(Level::INFO),
        fail_level: Some(Level::INFO),
    };

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/form", get(get_api_form).post(url_form))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(logger.clone())
                .on_response(logger.clone())
                .on_eos(())
                .on_body_chunk(())
                .on_failure(logger.clone()),
        );

    let server = Server::from_tcp(listener)?.serve(app.into_make_service());

    Ok(server)
}
