use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize};
use crate::mc::utils::command_builder::{CommandVersionConfig};

#[derive(Deserialize, Debug)]
pub struct JsonVersion {
    #[serde(skip)] pub minecraftArguments: (),
    #[serde(skip)] pub arguments: (),
    #[serde(default)] pub inheritsFrom: String,
    pub assetIndex: AssetsIndex,
    pub assets: String,
    #[serde(default)] pub compilanceLevel: u32,
    pub downloads: ClientDownloads,
    pub id: String,
    pub javaVersion: JavaVersion,
    pub libraries: Vec<Library>,
    pub mainClass: String,
    pub minimumLauncherVersion: u32,
    pub releaseTime: String,
    pub time: String,
    #[serde(rename = "type")] pub versionType: String,
    #[serde(skip)] pub logging: LogSettings
}

#[derive(Deserialize, Debug)]
pub struct AssetsIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub totalSize: u64,
    pub url: String
}
#[derive(Deserialize, Debug)]
pub struct ClientDownloads {
    pub client: Client,
    #[serde(default)] pub client_mappings: Client,
    #[serde(skip)] pub server: Client,
    #[serde(default)] pub server_mappings: Client
}
#[derive(Deserialize, Debug)]
#[derive(Default)]
pub struct Client {
    pub sha1: String,
    pub size: u64,
    pub url: String
}
#[derive(Deserialize, Debug)]
pub struct JavaVersion {
    pub component: String,
    pub majorVersion: u32,
}
pub fn default_vec_library_rules() -> Vec<LibraryRule> {
    vec![]
}
#[derive(Debug, Deserialize)]
pub struct LibraryDownloads {
    pub(crate) artifact: Option<LibraryDownloadsArtifacts>,
    pub(crate) classifiers: Option<HashMap<String ,LibraryDownloadsArtifacts>>,
}

#[derive(Debug, Deserialize)]
pub struct LibraryDownloadsArtifacts {
    pub(crate) path: String,
    sha1: String,
    pub size: u64,
    pub(crate) url: String,
}



#[derive(Debug, Deserialize)]
#[derive(Default)]
pub struct LibraryRuleOs {
    pub(crate) name: String,
    #[serde(skip)] version: String,
}

#[derive(Debug, Deserialize)]
pub struct LibraryRule {
    pub(crate) action: String,
    #[serde(default)] pub(crate) os: LibraryRuleOs,
}

#[derive(Debug, Deserialize)]
pub struct Library {
    pub(crate) downloads: Option<LibraryDownloads>,
    pub(crate) name: String,
    pub(crate) rules: Option<Vec<LibraryRule>>,
    #[serde(default)] pub(crate) url: String,
    pub natives: Option<LibraryNatives>,
    #[serde(default)] md5: String,
    #[serde(default)] sha1: String,
    #[serde(default)] sha256: String,
    #[serde(default)] sha521: String,
    #[serde(default)] size: String,
    pub(crate) extract: Option<LibraryExtract>,
}
#[derive(Deserialize, Debug)]
pub struct LibraryExtract {
    exclude: Vec<String>
}
#[derive(Deserialize, Debug, Default)]
pub struct LibraryNatives {
    pub osx: Option<String>,
    pub linux: Option<String>,
    pub windows: Option<String>
}
#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct LogSettings {
    pub client: LogSettingsClient
}
#[derive(Deserialize, Debug, Default)]
pub struct LogSettingsClient {
    pub argument: String,
    pub file: LogSettingsClientFile,
    #[serde(rename = "type")] pub ClientType: String
}
#[derive(Deserialize, Debug, Default)]
pub struct LogSettingsClientFile {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub url: String
}

pub fn load(file_str: &str) -> JsonVersion {
    let mut file = File::open(file_str).unwrap();
    let mut content = String::default();
    file.read_to_string(&mut content).expect("error");
    return serde_json::from_str(content.as_str()).expect("error");
}

impl JsonVersion {
    pub fn command_conf(&self) -> CommandVersionConfig {
        CommandVersionConfig {
            version_id: self.id.to_string(),
            version_type: self.versionType.to_string(),
            main_class: self.mainClass.to_string(),
        }
    }
}