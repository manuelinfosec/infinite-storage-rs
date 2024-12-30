use crate::args;
use crate::args::Arguments;

pub fn run_by_arguments(args: Arguments) -> anyhow::Result<()> {
    match args.command.expect("Command was not provided") {
        args::Commands::Embed(args) => {
            unimplemented!()
        }
        args::Commands::Dislodge(args) => {
            unimplemented!()
        }
        args::Commands::Download(args) => {
            unimplemented!()
        }
    }
}
