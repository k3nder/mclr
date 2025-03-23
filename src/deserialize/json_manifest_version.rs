use crate::deserialize::json_version;
use dwldutil::{DLBuilder, DLFile};
use serde::Deserialize;
use std::fs::File;
use std::io::Write;

#[derive(Deserialize, Debug)]
pub struct JsonVersion {
    pub id: String,
    #[serde(rename = "type")]
    pub versionType: String,
    pub url: String,
    pub time: String,
    pub releaseTime: String,
}
impl JsonVersion {
    pub fn save(&self, file: &str) {
        let dl = DLBuilder::new().add_file(DLFile::new().with_url(&self.url).with_path(file));
        dl.start();
    }
    pub fn save_and_load(&self, file: &str) -> json_version::JsonVersion {
        self.save(file);
        json_version::load(file)
    }
}
