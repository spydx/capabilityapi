use crate::helpers::initiate_app;

#[actix_rt::test]
async fn sample_test() {
    let app = initiate_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/users/", &app.address))
        .send()
        .await
        .expect("msg to be put here");

    assert!(response.status().is_success());
}