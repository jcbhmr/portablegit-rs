// #![cfg(windows)]
use std::{error::Error, process::Command};
use xshell::{Shell, cmd};

fn main() -> Result<(), Box<dyn Error>> {
    let sh = Shell::new()?;
    cmd!(sh, "7z x -aos ").run()?;

    Ok(())
}