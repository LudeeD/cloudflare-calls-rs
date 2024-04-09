use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum SdpType {
    Offer,
    Answer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionDescription {
    #[serde(rename = "type")]
    pub sdp_type: SdpType,
    pub sdp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TrackLocation {
    Local,
    Remote,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<TrackLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid: Option<String>,
    #[serde(rename = "trackName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_name: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TracksRequest {
    #[serde(rename = "sessionDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_description: Option<SessionDescription>,
    pub tracks: Vec<TrackObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TracksResponse {
    #[serde(rename = "requiresImmediateRenegotiation")]
    pub requires_immediate_renegotiation: bool,
    #[serde(rename = "sessionDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_description: Option<SessionDescription>,
    pub tracks: Vec<TrackObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewSessionRequest {
    #[serde(rename = "sessionDescription")]
    pub session_description: SessionDescription,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewSessionResponse {
    #[serde(rename = "sessionDescription")]
    pub session_description: SessionDescription,
    #[serde(rename = "sessionId")]
    pub session_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloseTracksRequest {
    #[serde(rename = "sessionDescription")]
    pub session_description: SessionDescription,
    pub tracks: Vec<TrackObject>,
    pub force: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloseTracksResponse {
    #[serde(rename = "requiresImmediateRenegotiation")]
    pub requires_immediate_renegotiation: bool,
    #[serde(rename = "sessionDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_description: Option<SessionDescription>,
    pub tracks: Vec<TrackObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionState {
    pub tracks: Vec<TrackObject>,
}
