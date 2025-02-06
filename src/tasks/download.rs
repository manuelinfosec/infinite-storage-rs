use youtube_dl::download_yt_dlp;
use std::process::Command;
use crate::args::DownloadParams;

/// Downloads a YouTube video using yt-dlp and saves it locally.
///
/// # Arguments
/// * `args` - A `DownloadParams` struct containing the URL of the video to download.
///
/// # Workflow
/// 1. Downloads and verifies the existence of the `yt-dlp` binary.
/// 2. Constructs a dynamic file name for the download based on the current timestamp.
/// 3. Executes the `yt-dlp` command to download the video.
/// 4. Outputs the result of the download operation.
///
/// # Returns
/// * `anyhow::Result<()>` - An empty result indicating success or an error if any step fails.
pub async fn run_download(args: DownloadParams) -> anyhow::Result<()> {
    // Step 1: Download and locate the yt-dlp binary.
    let yt_dlp_path = download_yt_dlp(".").await?;
    
    // Extract the video URL from the provided arguments.
    let url = args.url.expect("No URL in params when run_download");
    
    // Check if the yt-dlp path exists to ensure it was downloaded successfully.
    if !yt_dlp_path.exists() {
        println!("yt-dlp not found");
        return Ok(());
    }

    // Step 2: Create a unique output file name based on the current timestamp.
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let download_path = format!("downloaded_{}.mp4", timestamp);

    // Step 3: Start the download using the yt-dlp binary.
    println!("Starting the download, there is no progress bar");
    let output = Command::new(yt_dlp_path)
        .arg("-f")  // Specify video format.
        .arg("best") // Download the best available format.
        .arg("-o")  // Specify the output file path.
        .arg(download_path.clone()) // Output file path for the downloaded video.
        .arg(url)  // The URL to download the video from.
        .output()
        .expect("Failed to execute command");

    // Step 4: Check the result of the download command.
    if output.status.success() {
        // Successfully downloaded the video.
        println!("Video downloaded successfully");
        println!(
            "Output path: {}",
            std::fs::canonicalize(download_path).unwrap().display()
        );
    } else {
        // Failed to download the video. Display the error.
        println!("Video download failed");
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
