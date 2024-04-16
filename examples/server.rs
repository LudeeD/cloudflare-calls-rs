use cloudflare_calls_rs::{
    schema::{SdpType, SessionDescription},
    CallsApp,
};


#[tokio::main]
async fn main() {
    let calls_app = CallsApp::default();

    let remote_session_description = SessionDescription {
        sdp_type: SdpType::Offer,
        sdp: "demo".to_string(),
    };

    let new_session_response = calls_app
        .new_session(&remote_session_description)
        .await
        .expect("Failed to make request to Cloudflare Calls API");

    println!("Session created: {:?}", new_session_response.session_id);
}
