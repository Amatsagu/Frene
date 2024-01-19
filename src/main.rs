use clap::Parser;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

mod command;
mod cli;
mod util;

fn main() -> Result<(), util::error::FreneError> {
    let cli = cli::Cli::parse();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // #TODO - make proper error handler
    match cli.command {
        cli::Commands::Blur { file_path, strength } => command::blur_cmd_handler(&mut stdout, file_path, strength)?,
        cli::Commands::Scheme { file_path } => command::scheme_cmd_handler(&mut stdout, file_path)?,
    }

    stdout.set_color(&mut ColorSpec::new())?;
    Ok(())
}
