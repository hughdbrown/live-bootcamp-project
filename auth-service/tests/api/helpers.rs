use auth_service::Application;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let localhost = "127.0.0.1";
        let all_ports: i32 = 0;
        let app = Application::build(&localhost, all_ports)
            .await
            .expect("Failed to build app");

        let address: String = format!("http://{}", &app.address);

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread. 
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        // Create a Reqwest http client instance
        let http_client = reqwest::Client::new();

        // Create new `TestApp` instance and return it
        Self {address, http_client}
    }

    pub async fn get_root(&self) -> reqwest::Response {
        let url: String = format!("{}/", &self.address);
        self.http_client
            .get(&url)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        let url: String = format!("{}/signup", &self.address);
        self.http_client
            .post(&url)
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login(&self) -> reqwest::Response {
        let url: String = format!("{}/login", &self.address); 
        self.post(&url, "login").await
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        let url: String = format!("{}/logout", &self.address);
        self.post(&url, "logout").await
    }

    pub async fn post_verify_2fa(&self) -> reqwest::Response {
        let url: String = format!("{}/verify-2fa", &self.address);
        self.post(&url, "verify-2fa").await
    }

    pub async fn post_verify_token(&self) -> reqwest::Response {
        let url: String = format!("{}/verify-token", &self.address);
        self.post(&url, "verify-token").await
    }

    pub async fn post(&self, url: &str, post_type: &str) -> reqwest::Response {
        self.http_client
            .post(url)
            .send()
            .await
            .expect(&format!("Failed to execute {} request.", post_type))
    }
}


pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
