mod args;
mod etcher;
mod settings;
mod source;
mod tasks;
mod timer;
mod ui;

use clap::Parser;

/// The entry point of the application.
///
/// This tool, "Video Embedding System," allows users to encode files as
/// videos for storage on YouTube, which are later retrievable without data loss.
///
/// The user workflow includes:
/// 1. Zipping files to prepare for encoding.
/// 2. Using the embed option to convert the archive into a video.
/// 3. Transmit or store the generated video securely
/// 4. Downloading the video from YouTube when needed.
/// 5. Using the dislodge option to extract the original files from the video.
///
/// # Returns
/// * `anyhow::Result<()>` - Indicates success or failure during execution.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Welcome message explaining the tool's functionality.
    println!("Welcome to the Video Embedding System");
    println!("This system enables secure data transmission by converting files into a video format resistant to compression artifacts.");

    // Providing a structured guide for users
    println!("\nUsage Instructions:");
    println!(
        "1. Prepare your files by archiving them into a single compressed format (e.g., ZIP)."
    );
    println!("2. Use the 'Embed' option to encode the archive into a video file.");
    println!("3. Transmit or store the generated video securely.");
    println!("4. Use the 'Download' option to retrieve the video file.");
    println!("5. Use the 'Dislodge' option to extract the original files from the encoded video.");
    println!("6. Ensure data integrity and verify successful extraction.\n");

    println!("For optimal results, choose the appropriate encoding settings based on your security and efficiency requirements.");

    // Parse command-line arguments using the `Arguments` struct.
    let mut args = args::Arguments::parse();

    // Enhance the parsed arguments by interacting with the user through the UI.
    // This step may include prompting for missing arguments.
    let new_command = ui::enrich_arguments(args.command).await?;
    args.command = Some(new_command);

    // Execute the appropriate tasks based on the parsed and enriched arguments.
    tasks::run_by_arguments(args).await?;

    // Return success if everything went fine.
    Ok(())
}
