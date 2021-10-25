use axum::extract;
use axum::extract::Form;
use axum::http::StatusCode;
use daysquare_shared::Service;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn new_service(
    Form(input): Form<Service>,
    connection: extract::Extension<PgPool>,
) -> StatusCode {
    let request_span;
    let _request_span_guard;
    let connection = connection.0;
    request_span = tracing::info_span!(
        "Adding a new API service",
        request_url = %input.url
    );

    _request_span_guard = request_span.enter();

    tracing::event!(tracing::Level::INFO, "Recieved: {:?}", input);

    match sqlx::query!(
        r#"
        insert into daysquare.service (id, title, description, url)
        values ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        input.title,
        input.description,
        input.url
    )
    .execute(&connection)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
