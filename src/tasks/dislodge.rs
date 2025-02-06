use crate::{args::DislodgeParams, etcher};

/// Handles the "dislodge" operation, which extracts embedded data from a video file
/// and writes it back to a specified output path.
///
/// # Arguments
/// * `args` - Parameters for the dislodge operation, including the input and output file paths.
///
/// # Returns
/// * `anyhow::Result<()>` - Indicates success or failure during the dislodge process.
pub async fn run_dislodge(args: DislodgeParams) -> anyhow::Result<()> {
    // Extract embedded data from the input video file.
    // The function expects a valid input path to be provided.
    let out_data = etcher::read(
        &args
            .in_path
            .expect("Input path not provided for dislodge operation"),
        1, // Presumably, the frame index or processing flag for extraction
    )?;

    // Write the extracted data back to the specified output path.
    // Ensure the output path is valid and accessible.
    etcher::write_bytes(
        &args
            .out_path
            .expect("Output path not provided for dislodge operation"),
        out_data,
    )?;

    // Indicate successful completion of the dislodge operation.
    Ok(())
}
