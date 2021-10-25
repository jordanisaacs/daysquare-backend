mod helper;

#[tokio::test]
async fn health_check_works() {
    let app;
    let client;
    let response;
    app = helper::spawn_app().await;
    client = reqwest::Client::new();
    response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn new_service_returns_a_200_for_valid_form_data() {
    let app;
    let client;
    let response;
    let body;
    let saved;

    app = helper::spawn_app().await;
    client = reqwest::Client::new();
    body = "url=spotify.com&title=spotify&description=music+service";

    response = client
        .post(&format!("{}/service", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    saved = sqlx::query!("select title, url, description from daysquare.service",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.title, "spotify");
    assert_eq!(saved.url, "spotify.com");
    assert_eq!(saved.description, "music service");
}

#[tokio::test]
async fn new_service_returns_a_400_when_data_is_mission() {
    let app;
    let client;
    let test_cases;

    app = helper::spawn_app().await;
    client = reqwest::Client::new();
    test_cases = vec![
        ("url=spotify.com&description=music%service", "missing title"),
        ("title=spotify&description=music%service", "missing url"),
        ("url=spotify.com&title=spotify", "missing description"),
        ("description=hello%world", "missing title and url"),
        ("url=spotify.com", "missing title and description"),
        ("title=spotify", "missing url and description"),
        ("", "missing title, url, and description"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response;

        response = client
            .post(&format!("{}/service", &app.address))
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with the 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
