use crate::config::AppConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AppStatus {
    Idle,
    Recording,
    Downloading,
    Transcribing,
    Rewriting,
    Copying,
    Pasting,
    Ready,
    Error,
}

#[derive(Debug, Clone, Serialize)]
pub struct AppSnapshot {
    pub status: AppStatus,
    pub status_detail: String,
    pub capture_active: bool,
    pub config: AppConfig,
    pub api_key_present: bool,
    pub last_transcript: Option<String>,
    pub last_output: Option<String>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SettingsInput {
    pub gemini_enabled: bool,
    pub gemini_model: String,
    pub gemini_prompt: String,
    pub whisper_model_path: String,
    pub whisper_language: String,
    pub shortcut: String,
    pub auto_paste: bool,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProcessOutcome {
    pub raw_transcript: String,
    pub final_text: String,
    pub used_gemini: bool,
    pub clipboard_updated: bool,
    pub pasted: bool,
}
