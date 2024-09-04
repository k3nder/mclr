use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};
use serde::{Deserialize};
use crate::deserialize::json_manifest;
use crate::mc;
use crate::mc::utils::command_builder::{CommandVersionConfig};
use crate::utils::io_utils::system::OperatingSystem;
use crate::utils::manifest::manifest;

#[derive(Deserialize, Debug, Clone)]
pub struct JsonVersion {
    #[serde(skip)] pub minecraftArguments: (),
    #[serde(skip)] pub arguments: (),
    pub inheritsFrom: Option<String>,
    #[serde(default)] pub assetIndex: AssetsIndex,
    #[serde(default)] pub assets: String,
    #[serde(default)] pub compilanceLevel: u32,
    #[serde(default)] pub downloads: ClientDownloads,
    pub id: String,
    #[serde(default)] pub javaVersion: JavaVersion,
    pub libraries: Vec<Library>,
    pub mainClass: String,
    pub minimumLauncherVersion: Option<u32>,
    pub releaseTime: String,
    pub time: String,
    #[serde(rename = "type")] pub versionType: String,
    #[serde(default)] pub logging: Option<LogSettings>
}

#[derive(Deserialize, Debug, Clone)]
#[derive(Default)]
pub struct AssetsIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub totalSize: u64,
    pub url: String
}
#[derive(Deserialize, Debug, Clone)]
#[derive(Default)]
pub struct ClientDownloads {
    pub client: Client,
    #[serde(default)] pub client_mappings: Client,
    #[serde(skip)] pub server: Client,
    #[serde(default)] pub server_mappings: Client
}
#[derive(Deserialize, Debug, Clone)]
#[derive(Default)]
pub struct Client {
    pub sha1: String,
    pub size: u64,
    pub url: String
}
#[derive(Deserialize, Debug, Clone)]
#[derive(Default)]
pub struct JavaVersion {
    pub component: String,
    pub majorVersion: u32,
}
pub fn default_vec_library_rules() -> Vec<LibraryRule> {
    vec![]
}
#[derive(Debug, Deserialize, Clone)]
pub struct LibraryDownloads {
    pub(crate) artifact: Option<LibraryDownloadsArtifacts>,
    pub(crate) classifiers: Option<HashMap<String ,LibraryDownloadsArtifacts>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LibraryDownloadsArtifacts {
    pub(crate) path: String,
    pub(crate) sha1: String,
    pub size: u64,
    pub(crate) url: String,
}



#[derive(Debug, Deserialize, Clone)]
#[derive(Default)]
pub struct LibraryRuleOs {
    pub(crate) name: String,
    #[serde(skip)] version: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LibraryRule {
    pub(crate) action: String,
    #[serde(default)] pub(crate) os: LibraryRuleOs,
}

impl LibraryRule {
    pub fn allow(&self, os: &OperatingSystem) -> bool {
        if self.action.eq("allow") && self.os.name.eq(os.name()) {
            true
        } else if !self.action.eq("allow") && !self.os.name.eq(os.name()) {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
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
    #[serde(default)] size: usize,
    pub(crate) extract: Option<LibraryExtract>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct LibraryExtract {
    exclude: Vec<String>
}
#[derive(Deserialize, Debug, Default, Clone)]
pub struct LibraryNatives {
    pub osx: Option<String>,
    pub linux: Option<String>,
    pub windows: Option<String>
}
#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct LogSettings {
    pub client: LogSettingsClient
}
#[derive(Deserialize, Debug, Default, Clone)]
pub struct LogSettingsClient {
    pub argument: String,
    pub file: LogSettingsClientFile,
    #[serde(rename = "type")] pub ClientType: String
}
#[derive(Deserialize, Debug, Default, Clone)]
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
    let obj = serde_json::from_str(content.as_str());

    if obj.is_ok() {
        let mut o: JsonVersion = obj.unwrap();
        if let Some(ins) = o.inheritsFrom {
            let mut vers = manifest().get(ins.as_str()).unwrap().save_and_load(format!("{}.tmp", file_str).as_str());

            vers.libraries.append(&mut o.libraries);

            vers.id = o.id;
            vers.mainClass = o.mainClass;

            return vers
        }
        return o
    }
    obj.expect("s")
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