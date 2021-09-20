use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::helpers::TestApp;

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = TestApp::new().await;

    let body = "name=matt%20wilki&email=mattwilki17%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let response = app.post_subscriptions(body.into()).await;

    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn subscribe_persists_the_new_subscriber() {
    let app = TestApp::new().await;

    let body = "name=matt%20wilki&email=mattwilki17%40gmail.com";

    app.post_subscriptions(body.into()).await;

    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions;",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch subscription");

    assert_eq!(saved.email, "mattwilki17@gmail.com");
    assert_eq!(saved.name, "matt wilki");
    assert_eq!(saved.status, "pending_confirmation");
}

#[actix_rt::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    // Prepare
    let app = TestApp::new().await;
    let body = "name=matt%20wilki&email=mattwilki17%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;
    // Act
    app.post_subscriptions(body.into()).await;

    // Assert - Mock asserts on drop
}

#[actix_rt::test]
async fn subscribe_sends_a_confirmation_email_with_a_link() {
    let app = TestApp::new().await;
    let body = "name=matt%20wilki&email=mattwilki17%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];

    let confirmation_links = app.get_confirmation_links(email_request);
    assert_eq!(confirmation_links.html, confirmation_links.plain_text);
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = TestApp::new().await;
    let test_cases = vec![
        ("name=matt%20wilki", "missing email"),
        ("email=mattwilki17%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    let app = TestApp::new().await;
    let test_cases = vec![
        ("name=&email=testmail%40gmail.com", "empty name"),
        ("name=Matt&email=", "empty email"),
        ("name=Matt&email=not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = app.post_subscriptions(body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            description
        );
    }
}
