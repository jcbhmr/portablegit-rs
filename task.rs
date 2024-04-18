#!/usr/bin/env -S cargo +nightly -Zscript
```cargo
[dependencies]
octocrab = "0.38.0"
cargo_toml = "0.19.2"
reqwest = "0.12.3"
tokio = { version = "1.37.0", features = ["rt-multi-thread", "macros"] }
regex = "1.10.4"
```
async fn generate() -> Result<(), Box<dyn std::error::Error>> {
    use regex::Regex;
    let cargo = cargo_toml::Manifest::from_path("Cargo.toml")?;
    let octocrab = octocrab::instance();
    let cargo_package = cargo.package.ok_or("no package")?;
    let cargo_package_version = cargo_package.version();
    let local_gfw_version = cargo_package_version.split('+').collect::<Vec<_>>()[1];
    let latest_gfw_release = octocrab.repos("git-for-windows", "git").releases().get_latest().await?;
    let remote_gfw_version = latest_gfw_release.tag_name.trim_start_matches("v");
    eprintln!("local {local_gfw_version:?}");
    eprintln!("local {remote_gfw_version:?}");

    if local_gfw_version != remote_gfw_version {
        eprintln!("new git for windows version released");
        let mut version_parts = cargo_package_version.split('+').collect::<Vec<_>>()[0].split('.').collect::<Vec<_>>();
        let version_parts1_string = (version_parts[1].parse::<u32>()? + 1).to_string();
        version_parts[1] = version_parts1_string.as_str();
        version_parts[2] = "0";
        let new_version = version_parts.join(".") + "+" + remote_gfw_version;
        eprintln!("new version {new_version:?}");
        let mut cargo_toml_text = std::fs::read_to_string("Cargo.toml")?;
        cargo_toml_text = cargo_toml_text.replace(&format!("version = \"{cargo_package_version}\""), &format!("version = \"{new_version}\""));
        std::fs::write("Cargo.toml", cargo_toml_text)?;
        eprintln!("updated Cargo.toml version to {new_version:?}");
    }

    let gfw_version = remote_gfw_version;
    let git_version = Regex::new(r"^\d+\.\d+\.\d+")?.find(gfw_version).ok_or("no semver")?.as_str();
    eprintln!("gfw version {gfw_version:?}");
    eprintln!("git version {git_version:?}");

    let filename = format!("PortableGit-{git_version}-64-bit.7z.exe");
    let url = format!("https://github.com/git-for-windows/git/releases/download/v{gfw_version}/{filename}");
    eprintln!("url {url:?}");
    let response = reqwest::get(&url).await?;
    let content = response.bytes().await?;
    std::fs::write("PortableGit-64-bit.7z.exe", content)?;
    eprintln!("downloaded {url} to {}", "PortableGit-64-bit.7z.exe");

    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match std::env::args().nth(1).ok_or("no task")?.as_str() {
        "generate" => generate().await,
        _ => Err("no such task".into()),
    }
}
