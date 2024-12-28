use clap::{Args, Command, Parser, Subcommand, ValueEnum};

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
}
