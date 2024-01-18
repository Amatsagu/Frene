use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod command;

fn main() -> Result<(), std::io::Error> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
    writeln!(&mut stdout, "Hello world!")?;

    command::log::test(&mut stdout)?;

    stdout.set_color(&mut ColorSpec::new())?;
    Ok(())
}
