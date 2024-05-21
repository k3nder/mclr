use serde::Deserialize;
use crate::deserialize::json_manifest_version::JsonVersion;

#[derive(Deserialize)]
pub struct Manifest {
    pub latest: Latest,
    pub versions: Vec<JsonVersion>
}
#[derive(Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String
}