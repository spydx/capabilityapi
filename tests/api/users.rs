use crate::helpers::initiate_app;

#[actix_rt::test]
async fn no_token_access_rejected() {
    let app = initiate_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/users/", &app.address))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 401);
}

#[actix_rt::test]
async fn token_access_ok() {
    let app = initiate_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/users/", &app.address))
        .header("Authorization", "Bearer kenneth")
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
}
