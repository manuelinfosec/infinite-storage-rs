use serde::Deserialize;

#[derive(Deserialize)]
pub struct EmbedParams {
    pub in_path: Option<String>,
    pub preset: Option<EmbedPreset>,
    pub mode: Option<EmbedOutputMode>,
    pub block_size: Option<i32>,
    pub threads: Option<usize>,
    pub fps: Option<i32>,
    pub resolution: Option<String>,
}

#[derive(Deserialize)]
pub enum EmbedPreset {
    Optimal,
    Paranoid,
    MaxEfficiency,
}

#[derive(Deserialize)]
pub enum EmbedOutputMode {
    Colored,
    Binary,
}

// Placeholder for DownloadParams and DislodgeParams
#[derive(Deserialize)]
pub struct DownloadParams {
    // Define fields here
}

#[derive(Deserialize)]
pub struct DislodgeParams {
    // Define fields here
}
