use clap::{Args, Parser, Subcommand, ValueEnum};

/// Represents the top-level arguments parsed from the command line.
/// This struct contains a single optional `Commands` field that determines
/// which subcommand was invoked.
///
/// Example usage:
/// ```sh
/// my_app embed --in-path "data.txt" --preset optimal
/// ```
#[derive(Parser)]
pub struct Arguments {
    /// The subcommand to execute. Can be one of `Embed`, `Download`, or `Dislodge`.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Defines the available subcommands for the application.
#[derive(Subcommand)]
pub enum Commands {
    /// Subcommand for embedding data into a video.
    Embed(EmbedParams),

    /// Subcommand for downloading a video or other resources.
    Download(DownloadParams),

    /// Subcommand for extracting (dislodging) data from a video.
    Dislodge(DislodgeParams),
}

/// Presets for embedding data with different levels of compression resistance or efficiency.
/// These presets provide predefined configurations for convenience.
#[derive(Debug, Clone, ValueEnum)]
pub enum EmbedPreset {
    /// Optimal compression resistance.
    Optimal,

    /// Paranoid compression resistance, prioritizing maximum robustness against compression.
    Paranoid,

    /// Maximum efficiency for faster encoding or smaller file sizes.
    MaxEfficiency,
}

/// Output mode for embedding data, determining how the data is represented in the video.
/// Each mode has unique characteristics for handling compression.
#[derive(Debug, Clone, ValueEnum)]
pub enum EmbedOutputMode {
    /// Uses RGB values, resulting in vibrant colors, but the encoding is susceptible to compression artifacts.
    Colored,

    /// Uses black and white pixels for better resistance to compression, sacrificing color fidelity.
    Binary,
}

/// Parameters specific to the `embed` subcommand, which handles embedding data into a video.
/// All fields are optional, and defaults may be applied based on the user interface or runtime logic.
#[derive(Args, Default, Debug)]
pub struct EmbedParams {
    /// Path to the input file containing the data to be encoded into the video.
    /// Example: `"data.txt"`
    #[arg(short, long)]
    pub in_path: Option<String>,

    /// Preset for the embedding process.
    /// Allows selecting predefined configurations such as `Optimal`, `Paranoid`, or `MaxEfficiency`.
    #[arg(short, long)]
    pub preset: Option<EmbedPreset>,

    /// Mode for embedding data: `Colored` or `Binary`.
    /// This determines the visual and compression properties of the output.
    #[arg(long)]
    pub mode: Option<EmbedOutputMode>,

    /// Size of the block used for encoding data, specified as pixels per side.
    /// A smaller block size increases encoding density but may reduce compression resistance.
    #[arg(long)]
    pub block_size: Option<i32>,

    /// Number of threads to use for encoding.
    /// Increasing the thread count can improve performance on multi-core processors.
    #[arg(long)]
    pub threads: Option<usize>,

    /// Frames per second (FPS) for the output video.
    /// Higher FPS values result in smoother videos but may increase file size.
    #[arg(long)]
    pub fps: Option<i32>,

    /// Resolution of the output video.
    /// Valid options: `"144"`, `"240"`, `"360"`, `"480"`, `"720"`.
    /// Defaults to `"360"` if an invalid value is provided.
    #[arg(long)]
    pub resolution: Option<String>,
}

/// Parameters specific to the `download` subcommand, which handles downloading videos or other resources.
/// All fields are optional, and defaults may be applied based on the user interface or runtime logic.
#[derive(Args, Default)]
pub struct DownloadParams {
    /// URL of the resource to download.
    /// Example: `"https://example.com/video.mp4"`
    #[arg(short, long)]
    pub url: Option<String>,
}

/// Parameters specific to the `dislodge` subcommand, which handles extracting (dislodging) embedded data from a video.
/// All fields are optional, and defaults may be applied based on the user interface or runtime logic.
#[derive(Args, Default)]
pub struct DislodgeParams {
    /// Path to the input video file from which data will be extracted.
    /// Example: `"input.mp4"`
    #[arg(short, long)]
    pub in_path: Option<String>,

    /// Path to the output file where the extracted data will be saved (including the file extension).
    /// Example: `"output.txt"`
    #[arg(short, long)]
    pub out_path: Option<String>,
}
