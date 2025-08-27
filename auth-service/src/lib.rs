use std::error::Error;

use axum::{
    routing::{
        //get,
        post,
    },
    serve::Serve,
    Router,
    // response::Html,
};
use tower_http::services::ServeDir;

use routes::{
    login,
    logout,
    signup,
    verify_2fa,
    verify_token,
};
mod routes;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(
        server_addr: &str,
        server_port: i32,
    ) -> Result<Self, Box<dyn Error>> {
        // Here we are using ip 0.0.0.0 so the service is listening on all the configured network interfaces.
        // This is needed for Docker to work, which we will add later on.
        // See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working
        let address = format!("{server_addr}:{server_port}");

        // Move the Router definition from `main.rs` to here.
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token));

        // Application is running on port 3000:
        // ❯ sudo lsof -i :3000
        // COMMAND    PID      USER   FD   TYPE             DEVICE SIZE/OFF NODE NAME
        // com.docke 6606 hughbrown  174u  IPv6 0xa415fefe91b050ea      0t0  TCP *:hbci (LISTEN)
        // > ps aux | grep 6606
        // hughbrown         6606   ... /Applications/Docker.app/Contents/MacOS/com.docker.backend services --autostart
        let listener = tokio::net::TcpListener::bind(&address)
            .await
            .expect(&format!("Could not bind to address {address}\nRun `sudo lsof -i :{server_port}` to find app\n"));

        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}
