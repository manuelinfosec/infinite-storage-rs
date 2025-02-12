use anyhow; // Error handling library for cleaner error propagation
#[allow(unused_imports)] // Suppresses warnings for unused imports
use inquire::{min_length, Confirm, CustomType, MultiSelect, Password, Select, Text}; // Interactive command-line prompts from the 'inquire' crate

use crate::args::{Commands, DislodgeParams, DownloadParams, EmbedParams}; // Importing application-specific command parameters

/// Enriches and completes user-provided arguments by prompting for missing inputs.
///
/// This function handles three main commands: Embed, Download, and Dislodge.
/// If no command is provided, it prompts the user to select one.
///
/// # Arguments
/// * `args` - An optional set of user-provided command arguments.
///
/// # Returns
/// A fully enriched `Commands` variant based on user input.
pub async fn enrich_arguments(args: Option<Commands>) -> anyhow::Result<Commands> {
    // Handle existing command arguments or prompt the user for a new command.
    Ok(match args {
        Some(Commands::Embed(embed_args)) => {
            // Enrich Embed command parameters if provided
            Commands::Embed(enrich_embed_params(embed_args).await?)
        }
        Some(Commands::Download(download_args)) => {
            // Enrich Download command parameters if provided
            Commands::Download(enrich_download_params(download_args).await?)
        }
        Some(Commands::Dislodge(dislodge_args)) => {
            // Enrich Dislodge command parameters if provided
            Commands::Dislodge(enrich_dislodge_params(dislodge_args).await?)
        }
        None => {
            // Present user with available command options
            let options = vec!["Embed", "Download", "Dislodge"];

            let modes = Select::new("Pick what you want to do with the program", options)
                .with_help_message("Embed: Create a video from files,\n Download: Download files stored on YouTube,\n Dislodge: Return files from an embedded video")
                .prompt()
                .unwrap();

            match modes {
                "Embed" => Commands::Embed(enrich_embed_params(EmbedParams::default()).await?),
                "Download" => Commands::Download(enrich_download_params(DownloadParams::default()).await?),
                "Dislodge" => Commands::Dislodge(enrich_dislodge_params(DislodgeParams::default()).await?),
                _ => unreachable!(), // Ensures exhaustive matching
            }
        }
    })
}

/// Enriches the parameters for the Embed command by prompting the user for missing values.
async fn enrich_embed_params(mut args: EmbedParams) -> anyhow::Result<EmbedParams> {
    if args.in_path.is_none() {
        // Prompt user for input file path if not provided
        let path = Text::new("What is the path to your file ?")
            .with_default("src/tests/test.txt")
            .prompt()
            .unwrap();
        args.in_path = Some(path);
    }

    // println!("\nI couldn't figure out a weird bug that happens if you set the size to something that isn't a factor of the height");
    // println!("If you don't want the files you put in to come out as the audio/visual equivalent of a pipe bomb, account for the above bug\n");

    if args.mode.is_none()
        && args.block_size.is_none()
        && args.threads.is_none()
        && args.fps.is_none()
        && args.resolution.is_none()
    {
        // Offer preset options if no advanced parameters are set
        let presets = vec![
            "Optimal compression resistance",
            "Paranoid compression resistance",
            "Maximum efficiency",
            "Custom",
        ];
        let preset = Select::new("You can use one of the existing presets or custom settings", presets.clone())
            .with_help_message("Any amount of compression on Maximum Efficiency will corrupt all your hopes and dreams")
            .prompt()
            .unwrap();

        match preset {
            "Maximum efficiency" => {
                args.preset = Some(crate::args::EmbedPreset::MaxEfficiency);
                return Ok(args);
            }
            "Optimal compression resistance" => {
                args.preset = Some(crate::args::EmbedPreset::Optimal);
                return Ok(args);
            }
            "Paranoid compression resistance" => {
                args.preset = Some(crate::args::EmbedPreset::Paranoid);
                return Ok(args);
            }
            _ => (), // Custom settings fall through to advanced prompts
        }
    }

    // Custom or partially set parameters, prompting for each missing value
    if args.mode.is_none() {
        let out_modes = vec!["Colored", "B/W (Binary)"];
        let out_mode = Select::new("Pick how data will be embedded", out_modes.clone())
            .with_help_message("Colored mode is useless if the video undergoes compression at any point, B/W survives compression")
            .prompt()
            .unwrap();
        args.mode = Some(match out_mode {
            "Colored" => crate::args::EmbedOutputMode::Colored,
            "B/W (Binary)" => crate::args::EmbedOutputMode::Binary,
            _ => unreachable!(),
        });
    }

    if args.block_size.is_none() {
        let size = CustomType::<i32>::new("What size should the blocks be ?")
            .with_error_message("Please type a valid number")
            .with_help_message("Bigger blocks are more resistant to compression, I recommend 2-5.")
            .with_default(2)
            .prompt()?;
        args.block_size = Some(size);
    }

    if args.threads.is_none() {
        let threads = CustomType::<usize>::new("How many threads to dedicate for processing ?")
            .with_error_message("Please type a valid number")
            .with_help_message("The more threads, the merrier")
            .with_default(8)
            .prompt()?;
        args.threads = Some(threads);
    }

    if args.fps.is_none() {
        let fps = CustomType::<i32>::new("What fps should the video be at ?")
            .with_error_message("Please type a valid number")
            .with_help_message("Decreasing fps may decrease chance of compression. ~10fps works")
            .with_default(10)
            .prompt()
            .expect("Invalid fps");
        args.fps = Some(fps);
    }

    let resolutions = vec!["144p", "240p", "360p", "480p", "720p"];

    if args.resolution.is_none() {
        // Prompt user for video resolution
        let resolution = Select::new("Pick a resolution", resolutions)
            .with_help_message("I recommend 720p as the resolution won't affect compression")
            .prompt()
            .unwrap();
        args.resolution = Some(resolution.to_string());
    }

    Ok(args)
}

/// Enriches the parameters for the Download command by prompting the user for missing values.
async fn enrich_download_params(mut args: DownloadParams) -> anyhow::Result<DownloadParams> {
    if args.url.is_none() {
        // Prompt user for video URL if not provided
        let url = Text::new("What is the url to the video ?")
            .prompt()
            .unwrap();
        args.url = Some(url);
    }
    Ok(args)
}

/// Enriches the parameters for the Dislodge command by prompting the user for missing values.
async fn enrich_dislodge_params(mut args: DislodgeParams) -> anyhow::Result<DislodgeParams> {
    if args.in_path.is_none() {
        // Prompt user for input video path
        let in_path = Text::new("What is the path to your video ?")
            .with_default("output.avi")
            .prompt()
            .unwrap();
        args.in_path = Some(in_path);
    }

    if args.out_path.is_none() {
        // Prompt user for output file path
        let out_path = Text::new("Where should the output go ?")
            .with_help_message("Please include name of file and extension")
            .prompt()
            .unwrap();
        args.out_path = Some(out_path);
    }

    Ok(args)
}
