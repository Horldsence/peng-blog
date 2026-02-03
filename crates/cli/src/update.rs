use anyhow::{anyhow, Context, Result};
use console::style;
use flate2::read::GzDecoder;
use reqwest::Client;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::time::Duration;
use tar::Archive;
use tokio::io::AsyncWriteExt;

const GITHUB_REPO: &str = "Horldsence/peng-blog";
const GITHUB_API_BASE: &str = "https://api.github.com";

#[derive(Debug, Deserialize)]
pub struct GitHubRelease {
    tag_name: String,
    body: Option<String>,
    assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Deserialize)]
struct ReleaseAsset {
    name: String,
    browser_download_url: String,
}

pub async fn check_update() -> Result<Option<GitHubRelease>> {
    let client = Client::builder()
        .user_agent("peng-blog-update")
        .timeout(Duration::from_secs(30))
        .build()?;

    let url = format!(
        "{}/repos/{}/releases/latest",
        GITHUB_API_BASE, GITHUB_REPO
    );

    println!("{}", style("Checking for updates...").cyan());

    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to fetch release information")?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "GitHub API returned error: {}",
            response.status()
        ));
    }

    let release: GitHubRelease = response.json().await?;

    println!(
        "{} {}",
        style("Latest version:").green(),
        style(&release.tag_name).bold()
    );

    Ok(Some(release))
}

pub async fn download_update(url: &str, dest_path: &Path) -> Result<()> {
    let client = Client::builder()
        .user_agent("peng-blog-update")
        .timeout(Duration::from_secs(600))
        .build()?;

    println!(
        "{} {}",
        style("Downloading from:").cyan(),
        style(url).dim()
    );

    let response = client
        .get(url)
        .send()
        .await
        .context("Failed to download update")?;

    if !response.status().is_success() {
        return Err(anyhow!("Download failed with status: {}", response.status()));
    }

    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded = 0;

    let mut file = tokio::fs::File::create(dest_path)
        .await
        .context("Failed to create download file")?;

    let bytes = response
        .bytes()
        .await
        .context("Failed to download bytes")?;

    let chunk_len = bytes.len();
    file.write_all(&bytes)
        .await
        .context("Failed to write download chunk")?;
    downloaded += chunk_len as u64;

    if total_size > 0 {
        let progress = (downloaded as f64 / total_size as f64) * 100.0;
        print!(
            "\r{} {:.0}%",
            style("Progress:").cyan(),
            style(progress).green()
        );
        use std::io::Write;
        std::io::stdout().flush().ok();
    }

    println!();

    Ok(())
}

pub fn extract_archive(archive_path: &Path, dest_dir: &Path) -> Result<()> {
    println!(
        "{} {} -> {}",
        style("Extracting:").cyan(),
        style(archive_path.display()).dim(),
        style(dest_dir.display()).cyan()
    );

    let file = fs::File::open(archive_path).context("Failed to open archive")?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    archive
        .unpack(dest_dir)
        .context("Failed to extract archive")?;

    println!("{}", style("✓ Extraction complete").green());
    Ok(())
}

pub fn replace_files(
    extract_dir: &Path,
    target_dir: &Path,
    preserve_files: &[&str],
) -> Result<()> {
    println!(
        "{} {} -> {}",
        style("Installing files:").cyan(),
        style(extract_dir.display()).dim(),
        style(target_dir.display()).cyan()
    );

    let preserve_set: std::collections::HashSet<String> =
        preserve_files.iter().map(|s| s.to_string()).collect();

    for entry in walkdir::WalkDir::new(extract_dir)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let src_path = entry.path();

        if src_path.is_file() {
            let relative_path = src_path
                .strip_prefix(extract_dir)
                .context("Failed to get relative path")?;

            let dest_path = target_dir.join(relative_path);

            if let Some(filename) = dest_path.file_name().and_then(|n| n.to_str()) {
                if preserve_set.contains(filename) {
                    println!(
                        "{} {}",
                        style("  Preserving:").yellow(),
                        style(filename).dim()
                    );
                    continue;
                }
            }

            let dest_parent = dest_path
                .parent()
                .ok_or_else(|| anyhow!("No parent directory"))?;

            fs::create_dir_all(dest_parent)
                .context("Failed to create destination directory")?;

            fs::copy(src_path, &dest_path).context("Failed to copy file")?;

            println!(
                "{} {}",
                style("  Installed:").green(),
                style(relative_path.display()).dim()
            );
        }
    }

    println!("{}", style("✓ File replacement complete").green());
    Ok(())
}

pub async fn perform_update(force: bool) -> Result<()> {
    let current_exe = std::env::current_exe()?;

    println!(
        "\n{}\n{} {}\n{}\n",
        "═".repeat(60),
        style("Peng Blog Update"),
        style("(Self-Update)").dim(),
        "═".repeat(60)
    );

    let release = check_update()
        .await?
        .ok_or_else(|| anyhow!("No release information available"))?;

    if !force {
        println!(
            "\n{}: {}",
            style("Release notes").cyan(),
            style(
                release
                    .body
                    .as_ref()
                    .map(|s| s.lines().next().unwrap_or(""))
                    .unwrap_or("No release notes")
            )
            .dim()
        );
    }

    let temp_dir = tempfile::TempDir::new()?;
    let archive_name = format!("peng-blog-{}.tar.gz", release.tag_name);
    let archive_path = temp_dir.path().join(&archive_name);

    let asset = release
        .assets
        .iter()
        .find(|a| a.name.ends_with(".tar.gz"))
        .ok_or_else(|| anyhow!("No suitable archive found in release"))?;

    download_update(&asset.browser_download_url, &archive_path).await?;

    let extract_dir = temp_dir.path().join("extracted");
    fs::create_dir(&extract_dir)?;

    extract_archive(&archive_path, &extract_dir)?;

    let target_dir = current_exe
        .parent()
        .ok_or_else(|| anyhow!("Cannot determine executable directory"))?;

    let preserve_files = vec![".env", "config.toml", "uploads"];

    replace_files(&extract_dir, target_dir, &preserve_files)?;

    println!(
        "\n{}\n{}\n{}\n",
        "═".repeat(60),
        style("✓ Update completed successfully!").green().bold(),
        "═".repeat(60)
    );

    println!("{}", style("Next steps:").cyan().bold());
    println!("  1. Run database migrations:");
    println!("     {}", style("cargo run -- db migrate").yellow());
    println!("  2. Restart the server:");
    println!("     {}", style("cargo run").yellow());

    Ok(())
}
