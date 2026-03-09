use anyhow::Result;
use reqwest::{header::HeaderValue, Url};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GithubRelease {
    pub tag_name: String,
    pub prerelease: bool,
}

pub async fn fetch_latest_version() -> Result<Option<semver::Version>> {
    let client = reqwest::Client::new();
    let url = Url::parse("https://api.github.com/repos/rawnly/spm/releases/latest")?;

    let response = client
        .get(url)
        .header(
            "Accept",
            HeaderValue::from_static("application/vnd.github+json"),
        )
        .header("User-Agent", HeaderValue::from_static("spm"))
        .send()
        .await?;

    if response.status() == 404 {
        return Ok(None);
    }

    let release = response.json::<GithubRelease>().await?;

    if release.prerelease {
        return Ok(None);
    }

    Ok(semver::Version::parse(&release.tag_name).ok())
}
