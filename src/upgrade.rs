use flate2::read::GzDecoder;
use semver::Version;
use serde::Deserialize;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use tar::Archive;
use thiserror::Error;

const REPO: &str = "shanewwarren/cue";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Error, Debug)]
pub enum UpgradeError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] ureq::Error),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid version: {0}")]
    InvalidVersion(#[from] semver::Error),

    #[error("No release found")]
    NoRelease,

    #[error("No binary found for platform: {0}")]
    NoBinary(String),

    #[error("Could not determine executable path")]
    NoExecutablePath,
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub struct UpgradeInfo {
    pub current: Version,
    pub latest: Version,
    pub has_update: bool,
}

fn get_target() -> String {
    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        "unknown"
    };

    let os = if cfg!(target_os = "macos") {
        "apple-darwin"
    } else if cfg!(target_os = "linux") {
        "unknown-linux-gnu"
    } else {
        "unknown"
    };

    format!("{}-{}", arch, os)
}

fn fetch_latest_release() -> Result<Release, UpgradeError> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", REPO);

    let response: Release = ureq::get(&url)
        .set("User-Agent", "cue-cli")
        .call()?
        .into_json()?;

    Ok(response)
}

pub fn check_for_update() -> Result<UpgradeInfo, UpgradeError> {
    let release = fetch_latest_release()?;

    let current = Version::parse(CURRENT_VERSION)?;
    let latest_str = release.tag_name.trim_start_matches('v');
    let latest = Version::parse(latest_str)?;

    Ok(UpgradeInfo {
        has_update: latest > current,
        current,
        latest,
    })
}

pub fn perform_upgrade() -> Result<UpgradeInfo, UpgradeError> {
    let release = fetch_latest_release()?;

    let current = Version::parse(CURRENT_VERSION)?;
    let latest_str = release.tag_name.trim_start_matches('v');
    let latest = Version::parse(latest_str)?;

    if latest <= current {
        return Ok(UpgradeInfo {
            has_update: false,
            current,
            latest,
        });
    }

    let target = get_target();
    let asset_name = format!("cue-{}-{}.tar.gz", release.tag_name, target);

    let asset = release
        .assets
        .iter()
        .find(|a| a.name == asset_name)
        .ok_or_else(|| UpgradeError::NoBinary(target))?;

    // Download the tarball
    let response = ureq::get(&asset.browser_download_url)
        .set("User-Agent", "cue-cli")
        .call()?;

    let mut tarball = Vec::new();
    response.into_reader().read_to_end(&mut tarball)?;

    // Extract the binary
    let decoder = GzDecoder::new(&tarball[..]);
    let mut archive = Archive::new(decoder);

    let current_exe = env::current_exe().map_err(|_| UpgradeError::NoExecutablePath)?;
    let temp_path = current_exe.with_extension("new");

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;

        if path.file_name().map(|n| n == "cue").unwrap_or(false) {
            let mut file = File::create(&temp_path)?;
            io::copy(&mut entry, &mut file)?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&temp_path, fs::Permissions::from_mode(0o755))?;
            }

            break;
        }
    }

    // Replace the current binary
    let backup_path = current_exe.with_extension("old");

    // On Unix, we can rename while running
    if backup_path.exists() {
        fs::remove_file(&backup_path)?;
    }
    fs::rename(&current_exe, &backup_path)?;
    fs::rename(&temp_path, &current_exe)?;
    fs::remove_file(&backup_path)?;

    Ok(UpgradeInfo {
        has_update: true,
        current,
        latest,
    })
}
