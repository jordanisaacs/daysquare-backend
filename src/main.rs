use daysquare::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let subscriber = get_subscriber("daysquare".into(), "debug".into(), std::io::stdout);
    init_subscriber(subscriber);
    tracing::debug!("listening on 127.0.0.1:");

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    // Retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = daysquare::run(listener).expect("Failed to bind to address");

    tracing::debug!("listening on 127.0.0.1:{}", port);
    server.await
}
