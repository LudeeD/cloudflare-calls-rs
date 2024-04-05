use serde::Serialize;

struct CallsApp {
    pub app_id: String,
    app_secret: String,
    client: reqwest::Client,
    base_path: String,
}

impl CallsApp {
    pub fn new() -> CallsApp {
        let app_id = std::env::var("APP_ID").unwrap();
        let app_secret = std::env::var("APP_SECRET").unwrap();
        let base_path = "https://rtc.live.cloudflare.com/v1".to_string();

        CallsApp {
            app_id,
            app_secret,
            client: reqwest::Client::new(),
            base_path
        }
    }

    async fn send_request<T: Serialize + ?Sized>(&self, path: &str, method: reqwest::Method, body: Option<&T>) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", self.base_path, path);
        let mut request = self.client.request(method, &url);

        if let Some(body) = body {
            request = request.json(&body);
        }

        request.send().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // set  demo envs
        std::env::set_var("APP_ID", "123");
        std::env::set_var("APP_SECRET", "456");

        let app = CallsApp::new();

        assert_eq!(app.app_id, "123");
    }
}
