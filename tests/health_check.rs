use std::net::TcpListener;
use zero::run;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await.expect("Failed to spawn our app.");

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
// Launch our application in the background ~somehow~
async fn spawn_app() -> Result<String, std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr().unwrap().port();

    let server = run(listener)?;
    let _ = tokio::spawn(server);

    let address = format!("http://127.0.0.1:{}", port);
    println!("Server is running on {}", address);
    Ok(address)
}
