use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let port = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &port))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_when_valid_form_data() {
    let port = spawn_app();

    let client = reqwest::Client::new();

    let body = "name=john%20doe&email=john.doe%40gmail.com";

    let response = client
        .post(format!("{}/subscriptions", &port))
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
async fn subscribe_returns_400_when_invalid_form_data() {
    let port = spawn_app();

    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=john%20doe", "missing the email"),
        ("email=john.doe%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", &port))
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();
    let server = rustymail::run(listener).expect("Failed to run the Server");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
