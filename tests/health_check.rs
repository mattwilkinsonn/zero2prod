use std::net::TcpListener;

use reqwest;
use tokio;
#[actix_rt::test]
async fn health_check_works() {
    let addr = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &addr))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind listener");

    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to run server");

    let _ = tokio::spawn(server);

    format!("http://localhost:{}", port)
}
