use anyhow::Result;
use reqwest::{Method, Response};
use schema::{
    NewSessionRequest, NewSessionResponse, SessionDescription, TrackObject, TracksRequest,
    TracksResponse,
};
use serde::Serialize;

pub mod schema;

#[derive(Clone)]
pub struct CallsApp {
    pub app_id: String,
    app_secret: String,
    client: reqwest::Client,
    base_path: String,
}

impl Default for CallsApp {
    fn default() -> Self {
        let app_id = std::env::var("APP_ID").unwrap();
        let app_secret = std::env::var("APP_SECRET").unwrap();
        let base_path = "https://rtc.live.cloudflare.com/v1".to_string();
        let base_path = format!("{}/apps/{}", base_path, app_id);

        CallsApp {
            app_id,
            app_secret,
            client: reqwest::Client::new(),
            base_path,
        }
    }
}

impl CallsApp {
    pub async fn build_request<T: Serialize>(
        &self,
        path: &str,
        body: &T,
        method: Method,
    ) -> Result<Response> {
        let url = format!("{}/{}", self.base_path, path);

        let string_body = serde_json::to_string(body).unwrap();
        println!("string_body: {}", string_body);

        self.client
            .request(method, &url)
            .bearer_auth(&self.app_secret)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.into())
    }

    pub async fn new_session(&self, offer_sdp: &str) -> Result<NewSessionResponse> {
        let url = "sessions/new".to_string();
        let body = NewSessionRequest {
            session_description: SessionDescription {
                sdp_type: schema::SdpType::Offer,
                sdp: offer_sdp.to_string(),
            },
        };

        let response = self.build_request(&url, &body, Method::POST).await?;

        // check if the response code is 201
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to create new session"));
        }

        response
            .json::<NewSessionResponse>()
            .await
            .map_err(|e| e.into())
    }

    pub async fn add_tracks(
        &self,
        session_id: &str,
        track_objects: Vec<TrackObject>,
        offer_sdp: Option<String>,
    ) -> Result<TracksResponse> {
        let url = format!("sessions/{}/tracks/new", session_id);

        let body = match offer_sdp {
            Some(offer) => TracksRequest {
                session_description: Some(SessionDescription {
                    sdp_type: schema::SdpType::Offer,
                    sdp: offer,
                }),
                tracks: track_objects,
            },
            None => TracksRequest {
                session_description: None,
                tracks: track_objects,
            },
        };

        let response = self.build_request(&url, &body, Method::POST).await?;

        if !response.status().is_success() {
            let text = response.text().await?;
            return Err(anyhow::anyhow!(text));
        }

        response
            .json::<TracksResponse>()
            .await
            .map_err(|e| e.into())
    }

    pub async fn renegotiate(&self, session_id: &str, answer_sdp: &str) -> Result<()> {
        let url = format!("sessions/{}/renegotiate", session_id);
        let body = TracksRequest {
            session_description: Some(SessionDescription {
                sdp_type: schema::SdpType::Answer,
                sdp: answer_sdp.to_string(),
            }),
            tracks: vec![],
        };

        let response = self.build_request(&url, &body, Method::PUT).await?;

        if !response.status().is_success() {
            let text = response.text().await?;
            return Err(anyhow::anyhow!(text));
        }

        Ok(())
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

        let app = CallsApp::default();

        assert_eq!(app.app_id, "123");
    }
}
