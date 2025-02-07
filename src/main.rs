mod args;
mod etcher;
mod settings;
mod source;
mod tasks;
mod timer;
mod ui;

use clap::Parser;

// Embed route handler
/// The entry point of the application.
///
/// This tool, "Infinite Storage Glitch (ISG)," allows users to encode files as
/// videos for storage on YouTube, which are later retrievable without data loss.
///
/// The user workflow includes:
/// 1. Zipping files to prepare for encoding.
/// 2. Using the embed option to convert the archive into a video.
/// 3. Uploading the resulting video to YouTube.
/// 4. Downloading the video from YouTube when needed.
/// 5. Using the dislodge option to extract the original files from the video.
///
/// # Returns
/// * `anyhow::Result<()>` - Indicates success or failure during execution.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Welcome message explaining the tool's functionality.
    println!("Welcome to Video Embedding System");
    println!("This tool allows you to turn any file into a compression-resistant video that can be uploaded to YouTube for Infinite Storage:tm:");

    // Instructions for the user on how to use the tool.
    println!("\nHow to use:");
    println!("1. Zip all the files you will be uploading"); // Step 1: Prepare files
    println!("2. Use the embed option on the archive (THE VIDEO WILL BE SEVERAL TIMES LARGER THAN THE FILE, 4x in case of optimal compression resistance preset)"); // Step 2: Encode as video
    println!(
        "3. Upload the video to your YouTube channel. You probably want to keep it up as unlisted"
    ); // Step 3: Upload to YouTube
    println!("4. Use the download option to get the video back"); // Step 4: Download the video
    println!("5. Use the dislodge option to get your files back from the downloaded video"); // Step 5: Extract files
    println!("6. PROFIT\n"); // Step 6: Enjoy!

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
