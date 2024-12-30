use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Embed(EmbedParams),
    Download(DownloadParams),
    Dislodge(DislodgeParams),
}

#[derive(Debug, Clone, ValueEnum)]
pub enum EmbedPreset {
    /// Optimal compression resistance
    Optimal,
    /// Paranoid compression resistance
    Paranoid,
    /// Maximum efficiency
    MaxEfficiency,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum EmbedOutputMode {
    /// Uses RGB values and breaks under compression
    Colored,
    /// Uses black and white pixels and resists compression
    Binary,
}

/// This encodes the specific params foro embedding
/// All values are optional, and will be substituted in using UI if missing.
#[derive(Args, Default, Debug)]
pub struct EmbedParams {
    /// Path to the file with the data to encode
    #[arg(short, long)]
    pub in_path: Option<String>,

    /// Preset to use when encoding data.
    /// More specific encoding options override preset options
    #[arg(short, long)]
    pub preset: Option<EmbedPreset>,

    /// Etching mode
    #[arg(long)]
    pub mode: Option<EmbedOutputMode>,

    /// Block size, in pixels per side
    #[arg(long)]
    pub block_size: Option<i32>,

    /// Thread to use when encoding
    #[arg(long)]
    pub threads: Option<usize>,

    /// Output video FPS
    #[arg(long)]
    pub fps: Option<i32>,

    /// Output videoo resolution.
    /// Must be one oof "144", "240", "360", "480" or "720",
    /// and if the value provided is none of these, defaults
    /// to "360"
    #[arg(long)]
    pub resolution: Option<String>,
}

/// This encodes the specific params for downloading.
/// All values are optioal, and ill be substituted in using UI if missing
#[derive(Args, Default)]
pub struct DownloadParams {
    #[arg(short, long)]
    pub url: Option<String>,
}

/// This encodes the specific params for dislodginig.
/// All values are optional, and will be substituted in using UI if missing
#[derive(Args, Default)]
pub struct DislodgeParams {
    /// Path to input viideoo
    #[arg(short, long)]
    pub in_path: Option<String>,

    /// Path to file output (including extension)
    #[arg(short, long)]
    pub out_path: Option<String>,
}
