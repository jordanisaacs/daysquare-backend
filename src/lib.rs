use axum::{
    handler::{get, post},
    AddExtensionLayer, Router, Server,
};

use routes::*;

use sqlx::PgPool;
use std::future::Future;
use std::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::Level;

pub mod configuration;
mod domain;
//mod http;
pub mod routes;
pub mod telemetry;
pub mod tracelog;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<impl Future<Output = hyper::Result<()>>, hyper::Error> {
    let app;
    let logger;
    let server;

    let db_pool = AddExtensionLayer::new(db_pool);

    logger = tracelog::TracingLogger {
        req_level: Some(Level::INFO),
        _chunk_level: None,
        _eos_level: None,
        resp_level: Some(Level::INFO),
        fail_level: Some(Level::INFO),
    };

    app = Router::new()
        .route("/health_check", get(health_check))
        .route("/form", get(get_api_form).post(url_form))
        .route("/service", post(new_service))
        .layer(db_pool)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(logger.clone())
                .on_response(logger.clone())
                .on_eos(())
                .on_body_chunk(())
                .on_failure(logger.clone()),
        );

    server = Server::from_tcp(listener)?.serve(app.into_make_service());

    Ok(server)
}
