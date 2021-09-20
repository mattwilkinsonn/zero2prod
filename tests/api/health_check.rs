use crate::helpers::TestApp;

#[actix_rt::test]
async fn health_check_works() {
    let app = TestApp::new().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
