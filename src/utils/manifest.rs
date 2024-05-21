use reqwest::Error;
use crate::deserialize::json_manifest;
use crate::utils::io_utils::{get, get_string};
pub async fn manifest() -> json_manifest::Manifest {
    return serde_json::from_str(&*get_string("https://launchermeta.mojang.com/mc/game/version_manifest.json").await.expect("{ \"error\": 1 }")).unwrap();
}
