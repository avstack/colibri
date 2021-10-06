use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "colibriClass")]
pub enum ColibriMessage {
  #[serde(rename_all = "camelCase")]
  DominantSpeakerEndpointChangeEvent {
    dominant_speaker_endpoint: String,
    previous_speakers: Vec<String>,
  },
  #[serde(rename_all = "camelCase")]
  EndpointConnectivityStatusChangeEvent {
    endpoint: String,
    #[serde_as(as = "DisplayFromStr")]
    active: bool,
  },
  #[serde(rename_all = "camelCase")]
  EndpointMessage {
    from: String,
    to: Option<String>,
    msg_payload: serde_json::Value,
  },
  #[serde(rename_all = "camelCase")]
  EndpointStats {
    from: String,
    bitrate: Bitrates,
    packet_loss: PacketLoss,
    connection_quality: f32,
    #[serde(rename = "jvbRTT")]
    jvb_rtt: Option<u16>,
    server_region: String,
    max_enabled_resolution: u16,
  },
  #[serde(rename_all = "camelCase")]
  LastNChangedEvent { last_n: u16 },
  #[serde(rename_all = "camelCase")]
  LastNEndpointsChangeEvent { last_n_endpoints: Vec<String> },
  #[serde(rename_all = "camelCase")]
  ReceiverVideoConstraint { max_frame_height: u16 },
  #[serde(rename_all = "camelCase")]
  ReceiverVideoConstraints {
    last_n: Option<u16>,
    selected_endpoints: Option<Vec<String>>,
    on_stage_endpoints: Option<Vec<String>>,
    default_constraints: Option<Constraints>,
    constraints: Option<HashMap<String, Constraints>>,
  },
  #[serde(rename_all = "camelCase")]
  SelectedEndpointsChangedEvent { selected_endpoints: Vec<String> },
  #[serde(rename_all = "camelCase")]
  SenderVideoConstraints { video_constraints: Constraints },
  #[serde(rename_all = "camelCase")]
  ServerHello { version: Option<String> },
  #[serde(rename_all = "camelCase")]
  VideoTypeMessage { video_type: VideoType },
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VideoType {
  Camera,
  Desktop,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Constraints {
  pub ideal_height: Option<u16>,
  pub max_height: Option<u16>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bitrates {
  pub audio: Bitrate,
  pub video: Bitrate,
  #[serde(flatten)]
  pub total: Bitrate,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bitrate {
  pub upload: u32,
  pub download: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PacketLoss {
  pub total: u32,
  pub download: u32,
  pub upload: u32,
}
