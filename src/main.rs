use daysquare_backend::configuration::get_configuration;
use daysquare_backend::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let configuration;
    let connection_pool;
    let subscriber;
    let listener;
    let server;

    configuration = get_configuration().expect("Failed to read configuration.");
    connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    subscriber = get_subscriber("daysquare".into(), "debug".into(), std::io::stdout);
    init_subscriber(subscriber);

    listener =
        TcpListener::bind(configuration.server.public_addr()).expect("Failed to bind to address");

    server = daysquare_backend::run(listener, connection_pool)?;

    tracing::debug!(
        "listening on 127.0.0.1:{}",
        configuration.server.application_port()
    );
    server.await
}
