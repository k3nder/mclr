use crate::mc::utils::command_builder::CommandVersionConfig;
use crate::utils::io_utils::system::OperatingSystem;
use crate::utils::manifest::manifest;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug, Clone)]
pub struct JsonVersion {
    #[serde(skip, alias = "minecraftArguments")]
    pub minecraft_arguments: (),
    #[serde(skip)]
    pub arguments: (),
    #[serde(alias = "inheritsFrom")]
    pub inherits_from: Option<String>,
    #[serde(default, alias = "assetIndex")]
    pub asset_index: AssetsIndex,
    #[serde(default)]
    pub assets: String,
    #[serde(default, alias = "compilanceLevel")]
    pub compilance_level: u32,
    #[serde(default)]
    pub downloads: ClientDownloads,
    pub id: String,
    #[serde(default, alias = "javaVersion")]
    pub java_version: JavaVersion,
    pub libraries: Vec<Library>,
    #[serde(alias = "mainClass")]
    pub main_class: String,
    #[serde(alias = "minimumLauncherVersion")]
    pub minimum_launcher_version: Option<u32>,
    #[serde(alias = "releaseTime")]
    pub release_time: String,
    pub time: String,
    #[serde(alias = "type")]
    pub version_type: String,
    #[serde(default)]
    pub logging: Option<LogSettings>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct AssetsIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    #[serde(alias = "totalSize")]
    pub total_size: u64,
    pub url: String,
}
#[derive(Deserialize, Debug, Clone, Default)]
pub struct ClientDownloads {
    pub client: Client,
    #[serde(default)]
    pub client_mappings: Client,
    #[serde(skip)]
    pub server: Client,
    #[serde(default)]
    pub server_mappings: Client,
}
#[derive(Deserialize, Debug, Clone, Default)]
pub struct Client {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}
#[derive(Deserialize, Debug, Clone, Default)]
pub struct JavaVersion {
    pub component: String,
    #[serde(alias = "majorVersion")]
    pub major_version: u32,
}
pub fn default_vec_library_rules() -> Vec<LibraryRule> {
    vec![]
}
#[derive(Debug, Deserialize, Clone)]
pub struct LibraryDownloads {
    pub(crate) artifact: Option<LibraryDownloadsArtifacts>,
    pub(crate) classifiers: Option<HashMap<String, LibraryDownloadsArtifacts>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LibraryDownloadsArtifacts {
    pub(crate) path: String,
    pub(crate) sha1: String,
    pub size: u64,
    pub(crate) url: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct LibraryRuleOs {
    pub(crate) name: String,
    #[serde(skip)]
    version: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LibraryRule {
    pub(crate) action: String,
    pub(crate) os: Option<LibraryRuleOs>,
}

impl LibraryRule {
    pub fn allow(&self, os: &OperatingSystem) -> bool {
        if self.action.eq("allow") && self.os.is_none() {
            true
        } else if self.action.eq("allow") && self.os.clone().unwrap().name.eq(os.name()) {
            true
        } else if !self.action.eq("allow") && !self.os.clone().unwrap().name.eq(os.name()) {
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
    #[serde(default)]
    pub(crate) url: String,
    pub natives: Option<LibraryNatives>,
    #[serde(default)]
    md5: String,
    #[serde(default)]
    sha1: String,
    #[serde(default)]
    sha256: String,
    #[serde(default)]
    sha521: String,
    #[serde(default)]
    size: usize,
    pub(crate) extract: Option<LibraryExtract>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct LibraryExtract {
    exclude: Vec<String>,
}
#[derive(Deserialize, Debug, Default, Clone)]
pub struct LibraryNatives {
    pub osx: Option<String>,
    pub linux: Option<String>,
    pub windows: Option<String>,
}
#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct LogSettings {
    pub client: LogSettingsClient,
}
#[derive(Deserialize, Debug, Default, Clone)]
pub struct LogSettingsClient {
    pub argument: String,
    pub file: LogSettingsClientFile,
    #[serde(alias = "type")]
    pub client_type: String,
}
#[derive(Deserialize, Debug, Default, Clone)]
pub struct LogSettingsClientFile {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

pub fn load(file_str: &str) -> JsonVersion {
    let mut file = File::open(file_str).unwrap();
    let mut content = String::default();
    file.read_to_string(&mut content).expect("error");
    let obj = serde_json::from_str(content.as_str());

    if obj.is_ok() {
        let mut o: JsonVersion = obj.unwrap();
        if let Some(ins) = o.inherits_from {
            let mut vers = manifest()
                .get(ins.as_str())
                .unwrap()
                .save_and_load(format!("{}.tmp", file_str).as_str());

            vers.libraries.append(&mut o.libraries);

            vers.id = o.id;
            vers.main_class = o.main_class;

            return vers;
        }
        return o;
    }
    obj.expect("s")
}

impl JsonVersion {
    pub fn command_conf(&self) -> CommandVersionConfig {
        CommandVersionConfig {
            version_id: self.id.to_string(),
            version_type: self.version_type.to_string(),
            main_class: self.main_class.to_string(),
        }
    }
}
