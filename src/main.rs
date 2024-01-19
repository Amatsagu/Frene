use clap::Parser;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

mod command;
mod cli;

fn main() -> Result<(), std::io::Error> {
    let cli = cli::Cli::parse();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    match cli.command {
        cli::Commands::Blur { file_path } => command::blur_cmd_handler(&mut stdout, file_path),
        cli::Commands::Scheme { file_path } => command::scheme_cmd_handler(&mut stdout, file_path),
    }

    stdout.set_color(&mut ColorSpec::new())?;
    Ok(())
}
