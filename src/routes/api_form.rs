use axum::{extract::Form, response::Html};
use serde::Deserialize;

pub async fn get_api_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head>Submit API</head>
            <body>
                <form action="/form" method="post">
                    <label for="url">
                        Enter url:
                        <input type="text" name="url">
                    </label>

                    <input type="submit" value="Submit">
                </form>
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize, Debug)]
pub struct Input {
    url: String,
}

pub async fn url_form(Form(input): Form<Input>) {
    let request_span = tracing::info_span!(
        "Adding a new HTTP request.",
        request_url = %input.url,
    );

    let _request_span_guard = request_span.enter();

    dbg!(&input);
}
