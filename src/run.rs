use crate::args::Arguments;

pub mod dislodge;
pub mod download;
pub mod embed;

/// Executes the appropriate task based on the user's input arguments.
///
/// # Arguments
/// * `args` - Parsed command-line arguments encapsulated in the `Arguments` struct.
///
/// # Returns
/// * `anyhow::Result<()>` - Indicates success or failure during task execution.
pub async fn run_by_arguments(args: Arguments) -> anyhow::Result<()> {
    // Extract and match the provided command, or panic if none is provided.
    // It's expected that a command is always available when this function is called.
    match args
        .command
        .expect("Command was not provided by the time run_by_arguments is used")
    {
        // Handle the "Embed" command by invoking the embed module's function.
        crate::args::Commands::Embed(args) => embed::run_embed(args).await,

        // Handle the "Download" command by invoking the download module's function.
        crate::args::Commands::Download(args) => download::run_download(args).await,

        // Handle the "Dislodge" command by invoking the dislodge module's function.
        crate::args::Commands::Dislodge(args) => dislodge::run_dislodge(args).await,
    }
}
