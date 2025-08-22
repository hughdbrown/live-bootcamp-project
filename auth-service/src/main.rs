use auth_service::Application;

#[tokio::main]
async fn main() {
    let server_addr = "0.0.0.0";
    let server_port = 8000;
    let app = Application::build(server_addr, server_port)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
