use alvr_common::{info, DeviceMotion, LogEntry, Pose};
use alvr_packets::{AudioDevicesList, ButtonValue};
use alvr_session::SessionConfig;
use csv::Writer;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::{path::PathBuf, time::Duration};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct StatisticsSummary {
    pub video_packets_total: usize,
    pub video_packets_per_sec: usize,
    pub video_mbytes_total: usize,
    pub video_mbits_per_sec: f32,
    pub total_latency_ms: f32,
    pub network_latency_ms: f32,
    pub encode_latency_ms: f32,
    pub decode_latency_ms: f32,
    pub packets_lost_total: usize,
    pub packets_lost_per_sec: usize,
    pub client_fps: u32,
    pub server_fps: u32,
    pub battery_hmd: u32,
    pub hmd_plugged: bool,
}

// Bitrate statistics minus the empirical output value
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct NominalBitrateStats {
    pub scaled_calculated_bps: Option<f32>,
    pub decoder_latency_limiter_bps: Option<f32>,
    pub network_latency_limiter_bps: Option<f32>,
    pub encoder_latency_limiter_bps: Option<f32>,
    pub manual_max_bps: Option<f32>,
    pub manual_min_bps: Option<f32>,
    pub requested_bps: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct GraphStatistics {
    pub total_pipeline_latency_s: f32,
    pub game_time_s: f32,
    pub server_compositor_s: f32,
    pub encoder_s: f32,
    pub network_s: f32,
    pub decoder_s: f32,
    pub decoder_queue_s: f32,
    pub client_compositor_s: f32,
    pub vsync_queue_s: f32,
    pub client_fps: f32,
    pub server_fps: f32,
    pub nominal_bitrate: NominalBitrateStats,
    pub actual_bitrate_bps: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrackingEvent {
    pub target_timestamp: Duration,
    pub device_motions: Vec<(String, DeviceMotion)>,
    pub hand_skeletons: [Option<[Pose; 26]>; 2],
    pub eye_gazes: [Option<Pose>; 2],
    pub fb_face_expression: Option<Vec<f32>>,
    pub htc_eye_expression: Option<Vec<f32>>,
    pub htc_lip_expression: Option<Vec<f32>>,
}

impl TrackingEvent {
    pub fn to_csv(&self, path: &str) -> Result<(), Box<dyn Error>> {
        // Open the file in append mode
        let file = OpenOptions::new().append(true).create(true).open(path)?;
        // Write the header only if the file is newly created
        if file.metadata()?.len() == 0 {
            let mut wtr = Writer::from_writer(&file);
            wtr.write_record(&["target_timestamp", "device_id", "motion_data"])?;
            // Adjust field names as needed
        }
        // Serialize each entry in device_motions and write to CSV
        for (device_id, motion_data) in &self.device_motions {
            let serialized_motion_data = serde_json::to_string(motion_data)?;
            let timestamp = self.target_timestamp.as_millis().to_string();
            let mut wtr = Writer::from_writer(&file);
            wtr.write_record(&[&timestamp, device_id, &serialized_motion_data])?;
        }
        // // Add similar code blocks for other fields
        // wtr.write_record(&[
        //     "hand_skeletons",
        //     "eye_gazes",
        //     "fb_face_expression",
        //     "htc_eye_expression",
        //     "htc_lip_expression",
        // ])?;
        // wtr.write_record(&[
        //     format!("{:?}", self.hand_skeletons),
        //     format!("{:?}", self.eye_gazes),
        //     format!("{:?}", self.fb_face_expression),
        //     format!("{:?}", self.htc_eye_expression),
        //     format!("{:?}", self.htc_lip_expression),
        // ])?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ButtonEvent {
    pub path: String,
    pub value: ButtonValue,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HapticsEvent {
    pub path: String,
    pub duration: Duration,
    pub frequency: f32,
    pub amplitude: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "id", content = "data")]
pub enum EventType {
    Log(LogEntry),
    Session(Box<SessionConfig>),
    StatisticsSummary(StatisticsSummary),
    GraphStatistics(GraphStatistics),
    Tracking(Box<TrackingEvent>),
    Buttons(Vec<ButtonEvent>),
    Haptics(HapticsEvent),
    AudioDevices(AudioDevicesList),
    DriversList(Vec<PathBuf>),
    ServerRequestsSelfRestart,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub timestamp: String,
    pub event_type: EventType,
}

pub fn send_event(event_type: EventType) {
    info!("{}", serde_json::to_string(&event_type).unwrap());
}
