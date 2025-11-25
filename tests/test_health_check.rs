mod common;

#[tokio::test]
async fn health_check_works() {
    let url = common::spawn_app();
    let client = reqwest::Client::new();

    let url = format!("{}/health_check", url);

    let response = client
        .get(&url)
        .send()
        .await
        .expect("Failed to execute target");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
