use crate::{
    args::{EmbedOutputMode, EmbedParams, EmbedPreset},
    etcher,
    settings::{Data, OutputMode, Settings},
};

/// Handles the embedding operation by configuring settings based on user input or defaults,
/// and then processing the input data to create an output video.
///
/// # Arguments
/// * `args` - Parameters for the embedding operation, including optional presets, resolution, block size, etc.
///
/// # Returns
/// * `anyhow::Result<()>` - Returns `Ok(())` if successful, or an error wrapped in `anyhow::Error` if something fails.
pub async fn run_embed(args: EmbedParams) -> anyhow::Result<()> {
    // Initialize settings with default values
    let mut settings: Settings = Settings::default();
    // Default output mode is set to Binary
    let mut output_mode = OutputMode::Binary;

    // Configure settings based on the preset, if provided
    match args.preset {
        Some(EmbedPreset::MaxEfficiency) => {
            // MaxEfficiency preset prioritizes speed and lower resource usage
            output_mode = OutputMode::Color;
            settings.size = 1;         // Smaller block size
            settings.threads = 8;      // Use 8 threads
            settings.fps = 10.0;       // Lower FPS
            settings.width = 256;      // Smaller video resolution width
            settings.height = 144;     // Smaller video resolution height
        }
        Some(EmbedPreset::Optimal) => {
            // Optimal preset balances quality and efficiency
            output_mode = OutputMode::Binary;
            settings.size = 2;         // Moderate block size
            settings.threads = 8;      // Use 8 threads
            settings.fps = 10.0;       // Moderate FPS
            settings.width = 1280;     // HD resolution width
            settings.height = 720;     // HD resolution height
        }
        Some(EmbedPreset::Paranoid) => {
            // Paranoid preset prioritizes robustness and maximum data redundancy
            output_mode = OutputMode::Binary;
            settings.size = 4;         // Larger block size
            settings.threads = 8;      // Use 8 threads
            settings.fps = 10.0;       // Moderate FPS
            settings.width = 1280;     // HD resolution width
            settings.height = 720;     // HD resolution height
        }
        None => {
            // If no preset is provided, settings will remain at their default values
        }
    }

    // If resolution is not set by the preset or arguments, fallback to default resolution
    if settings.width == 0 || settings.height == 0 {
        if args.resolution.is_none() {
            // Default resolution if none is provided
            settings.width = 640;      // Default width
            settings.height = 360;     // Default height
        } else {
            // Parse resolution from the provided string argument
            let (width, height) = match args.resolution.unwrap().as_str() {
                "144p" => (256, 144),
                "240p" => (426, 240),
                "360p" => (640, 360),
                "480p" => (854, 480),
                "720p" => (1280, 720),
                _ => (640, 360), // Default to 360p if resolution is invalid
            };
            settings.width = width;
            settings.height = height;
        }
    }

    // Override output mode if explicitly provided in arguments
    if let Some(mode) = args.mode {
        output_mode = mode.into();
    }

    // Override block size if explicitly provided
    if let Some(bs) = args.block_size {
        settings.size = bs;
    }

    // Override thread count if explicitly provided
    if let Some(threads) = args.threads {
        settings.threads = threads;
    }

    // Override FPS if explicitly provided
    if let Some(fps) = args.fps {
        settings.fps = fps.into();
    }

    // Match the output mode to perform the embedding operation
    match output_mode {
        OutputMode::Color => {
            // Handle color output mode
            // Rip the raw bytes from the input file
            let bytes = etcher::rip_bytes(&args.in_path.expect("No path provided in arguments"))?;

            // Create data in color mode
            let data = Data::from_color(bytes);

            // Perform the etching operation to generate the output video
            etcher::etch("output.avi", data, settings)?;
        }
        OutputMode::Binary => {
            // Handle binary output mode
            // Rip the raw bytes from the input file
            let bytes = etcher::rip_bytes(&args.in_path.expect("No path provided in arguments"))?;
            // Convert raw bytes to binary format
            let binary = etcher::rip_binary(bytes)?;

            // Create data in binary mode
            let data = Data::from_binary(binary);

            // Perform the etching operation to generate the output video
            etcher::etch("output.avi", data, settings)?;
        }
    }

    // Return success
    Ok(())
}
