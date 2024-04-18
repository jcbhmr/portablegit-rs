use std::{env::{current_dir, var}, error::Error, path::{Path, PathBuf}};
use xshell::{Shell, cmd};

fn main() -> Result<(), Box<dyn Error>> {
    let archive = current_dir()?.join("PortableGit-64-bit.7z.exe");
    let xdg_dirs = xdg::BaseDirectories::with_prefix("portablegit-rs")?;
    let dest = xdg_dirs.create_data_directory(env!("CARGO_PKG_VERSION"))?;
    if !dest.exists() {
        let sh = Shell::new()?;
        sh.change_dir(dest);
        cmd!(sh, "7z x -aos {archive}").run()?;    
    }
    Ok(())
}
