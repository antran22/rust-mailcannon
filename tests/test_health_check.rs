pub mod common;

#[tokio::test]
async fn health_check_works() {
    let test_app = common::spawn_app().await;
    let client = reqwest::Client::new();

    let url = format!("{}/health_check", &test_app.address);

    let response = client
        .get(&url)
        .send()
        .await
        .expect("Failed to execute target");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
