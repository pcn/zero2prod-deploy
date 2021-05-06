// tests/health-check.rs
// use std::net::TcpListener;
// use zero2prod::startup::run;

// fn spawn_app() -> String {
//     let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
//     let port = listener.local_addr().unwrap().port();
//     let server = run(listener).expect("Failed to bind address");
//     let _ = tokio::spawn(server);
//     format!("http://127.0.0.1:{}", port) // Return the port?
// }

// #[actix_rt::test]
// async fn health_check_works() {
//     // Arrange
//     let address = spawn_app();
//     let client = reqwest::Client::new();

//     let response = client
//         .get(&format!("{}/health_check", &address))
//         .send()
//         .await
//         .expect("Failed to execute request");

//     println!("Response status is {:?}", response.status());
//     assert!(response.status().is_success());
//     assert_eq!(Some(0), response.content_length());
// }

// #[actix_rt::test]
// async fn subscribe_returns_a_200_for_valid_form_data() {
//     // Arrange
//     let app_address = spawn_app();
//     let client = reqwest::Client::new();
//     let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

//     // Act
//     let response = client
//         .post(&format!("{}/subscriptions", &app_address))
//         .header("Content-type", "application/x-www-form-urlencoded")
//         .body(body)
//         .send()
//         .await
//         .expect("Failed to execute request.");

//     // Assert
//     assert_eq!(200, response.status().as_u16());
// }

// #[actix_rt::test]
// async fn susbscribe_returns_a_400_when_data_is_missing() {
//     // Arrange
//     let app_address = spawn_app();
//     let client = reqwest::Client::new();
//     let test_cases = vec![
//         ("name=le%20guin", "missing the email"),
//         ("email=ursula_le_guin%40gmail.com", "missing the name"),
//         ("", "missing both name and email"),
//     ];

//     for (invalid_body, error_message) in test_cases {
//         // Act
//         let response = client
//             .post(&format!("{}/subscriptions", &app_address))
//             .header("Content-Type", "application/x-www-form-urlencoded")
//             .body(invalid_body)
//             .send()
//             .await
//             .expect("Failed to execute reuest.");
//         // Assert
//         assert_eq!(
//             400,
//             response.status().as_u16(),
//             // Additional customized error message on test failure
//             "The API did not fail with a 400 Bad Request when the payload was {}.",
//             error_message
//         );
//     }
// }

async fn spawn_app() -> std::io::Result<()> {
    zero2prod::run().await 
}

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await.expect("Failed to spawn our app.");
    // We need to bring in `reqwest`
    // to perform HTTP requests against our applicatoin.
    //
    // Use `cargo add reqwest --dev --version 0.11` to add it under
    // `[dev-dependencies]` in Cargo.toml
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
async fn spawn_app() -> std::io::Request<()> {
    todo!()
}
      
