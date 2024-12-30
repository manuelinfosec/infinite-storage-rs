use crate::args::Arguments;

pub fn run_by_arguments(args: Arguments) -> anyhow::Result<()> {
    match args.command.expect("Command was not provided") {
        crate::args::Commands::Embed(args) => {
            unimplemented!()
        }
        crate::args::Commands::Dislodge(args) => {
            unimplemented!()
        }
        crate::args::Commands::Download(args) => {
            unimplemented!()
        }
    }
}
