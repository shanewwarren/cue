mod archive;
mod cli;
mod config;
mod playback;
mod upgrade;

use archive::{ArchiveError, SoundArchive};
use clap::Parser;
use cli::{Cli, Command};
use config::Config;
use playback::Player;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {e}");
        return ExitCode::from(1);
    }

    ExitCode::SUCCESS
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    // Handle upgrade separately (doesn't need config/archive)
    if let Command::Upgrade { check } = cli.command {
        return run_upgrade(check);
    }

    let config = Config::load()?;
    let archive = SoundArchive::load(&config.sounds_path)?;

    match cli.command {
        Command::Play { category, volume } => {
            let cat = archive
                .category(&category)
                .ok_or_else(|| ArchiveError::CategoryNotFound(category.clone()))?;

            let sound = cat
                .random()
                .ok_or_else(|| ArchiveError::EmptyCategory(category.clone()))?;

            let player = Player::new()?;
            let vol = volume as f32 / 100.0;
            player.play(&sound.path, vol)?;
        }

        Command::List { category } => {
            if let Some(cat_name) = category {
                let cat = archive
                    .category(&cat_name)
                    .ok_or_else(|| ArchiveError::CategoryNotFound(cat_name.clone()))?;

                println!("Sounds in '{}':", cat.name);
                for sound in &cat.sounds {
                    println!("  {}", sound.name);
                }
            } else {
                println!("Available categories:");
                for cat in &archive.categories {
                    let count = cat.sounds.len();
                    let plural = if count == 1 { "sound" } else { "sounds" };
                    println!("  {} ({} {})", cat.name, count, plural);
                }
            }
        }

        Command::Preview {
            category,
            sound,
            volume,
        } => {
            let cat = archive
                .category(&category)
                .ok_or_else(|| ArchiveError::CategoryNotFound(category.clone()))?;

            let snd = cat.sound(&sound).ok_or_else(|| {
                ArchiveError::SoundNotFound(format!("'{}' in category '{}'", sound, category))
            })?;

            let player = Player::new()?;
            let vol = volume as f32 / 100.0;
            player.play(&snd.path, vol)?;
        }

        Command::Upgrade { .. } => unreachable!(),
    }

    Ok(())
}

fn run_upgrade(check_only: bool) -> Result<(), Box<dyn std::error::Error>> {
    if check_only {
        let info = upgrade::check_for_update()?;

        if info.has_update {
            println!("Update available: v{} -> v{}", info.current, info.latest);
            println!("Run 'cue upgrade' to install");
        } else {
            println!("Already up to date (v{})", info.current);
        }
    } else {
        println!("Checking for updates...");

        let info = upgrade::perform_upgrade()?;

        if info.has_update {
            println!("Upgraded: v{} -> v{}", info.current, info.latest);
        } else {
            println!("Already up to date (v{})", info.current);
        }
    }

    Ok(())
}
