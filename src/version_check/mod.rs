use anyhow::Result;
use semver::Version;

pub mod github_api;

pub async fn is_update_available() -> Result<Option<Version>> {
    match github_api::fetch_latest_version().await? {
        Some(latest_release) => {
            let installed_version = Version::parse(env!("CARGO_PKG_VERSION"))?;

            if installed_version >= latest_release {
                return Ok(None);
            }

            Ok(Some(latest_release))
        }
        None => Ok(None),
    }
}
