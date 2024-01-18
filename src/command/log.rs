use std::io::Write;
use termcolor::StandardStream;

pub fn test(stdout: &mut StandardStream) -> Result<(), std::io::Error> {
    writeln!(stdout, "Some extra test from another module!")?;
    Ok(())
}