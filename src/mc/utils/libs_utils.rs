use dwldutil::decompress::{DLDecompressionConfig, DecompressionMethod};
use dwldutil::{DLBuilder, DLFile};
use log::debug;

use crate::deserialize::json_version::{Library, LibraryDownloads, LibraryNatives, LibraryRule};
use crate::utils::io_utils::get_resource_name;
use crate::utils::io_utils::system::OperatingSystem;
use crate::utils::{CounterEvent, HandleEvent};

struct MavenLibrary {
    pub group_id: String,
    pub artifact_id: String,
    pub version: String,
    pub repository: String,
}

impl MavenLibrary {
    pub fn parse(name: String, repository: String) -> Self {
        let tokens: Vec<&str> = name.split(":").collect();

        MavenLibrary {
            repository,
            group_id: tokens.get(0).unwrap().to_string(),
            artifact_id: tokens.get(1).unwrap().to_string(),
            version: tokens.get(2).unwrap().to_string(),
        }
    }

    pub fn all_url(&self) -> String {
        let group = self.group_id.replace(".", "/");
        format!(
            "{}{}/{}/{}/{}",
            self.repository,
            group,
            self.artifact_id,
            self.version,
            self.cl_name()
        )
    }

    pub fn cl_name(&self) -> String {
        format!("{}-{}.jar", self.artifact_id, self.version)
    }
}

pub fn filter_libs(
    destination: &str,
    binary_destination: &str,
    libs: &Vec<Library>,
    event: HandleEvent<CounterEvent>,
) -> Result<DLBuilder, String> {
    let mut index = 0;
    let mut filtered_files: Vec<DLFile> = Vec::new();
    for lib in libs {
        debug!("Checking... {}", &lib.clone().name.as_str());
        let natives = &&lib.clone().natives;
        if let Some(downloads) = &lib.clone().downloads {
            // artifact
            debug!("Downloading as artifact...");
            match artifact_download(destination, &lib, &downloads) {
                Ok(file) => filtered_files.push(file),
                Err(e) => debug!("Error downloading artifact: {}", e),
            }
            // classfiers
            debug!("Downloading as classifier...");
            match classifier_download(destination, binary_destination, natives, &downloads) {
                Ok(file) => filtered_files.push(file),
                Err(e) => debug!("Error downloading classifier: {}", e),
            }
        } else {
            let lib = MavenLibrary::parse(lib.clone().name, lib.clone().url);
            filtered_files.push(
                DLFile::new()
                    .with_url(lib.all_url().as_str())
                    .with_path(format!("{}/{}", destination, lib.cl_name().as_str()).as_str()),
            );
        }
        index += 1;
        event.event(CounterEvent::new(libs.len(), index));
    }
    Ok(DLBuilder::from_files(filtered_files))
}

fn classifier_download(
    destination: &str,
    binary_destination: &str,
    natives: &&Option<LibraryNatives>,
    downloads: &&LibraryDownloads,
) -> Result<DLFile, String> {
    let clc = &downloads.classifiers;
    if !clc.is_none() {
        let native_key = get_natives_value(natives);
        debug!("Find native classifier... {}", native_key.as_str());
        if let Some(n) = &clc.clone().unwrap().get(&native_key) {
            debug!("Download allowed...");
            let file = format!(
                "{}/{}",
                destination,
                get_resource_name(&n.url).unwrap().as_str()
            );
            return Ok(DLFile::new()
                .with_url(&n.url)
                .with_path(&file)
                .with_size(n.size)
                .with_decompression_config(
                    DLDecompressionConfig::new(DecompressionMethod::Zip, binary_destination)
                        .with_delete_after(false),
                ));
        } else {
            return Err("Download failed... No native classifier found".to_string());
        }
    } else {
        return Err("No classifiers on lib...".to_string());
    }
}

fn artifact_download(
    destination: &str,
    lib: &&Library,
    downloads: &&LibraryDownloads,
) -> Result<DLFile, String> {
    if let Some(a) = &downloads.artifact {
        let file = format!(
            "{}/{}",
            destination,
            get_resource_name(&a.clone().url).unwrap().as_str()
        );
        if let Some(r) = &lib.rules {
            if find_out_os(r) {
                return Ok(DLFile::new()
                    .with_url(&a.clone().url)
                    .with_path(&file)
                    .with_size(a.clone().size));
            } else {
                return Err(format!("Not Allow by OS... {}", file));
            }
        } else {
            debug!("Allow by no rules... {}", file);
            return Ok(DLFile::new()
                .with_url(&a.clone().url)
                .with_path(
                    format!(
                        "{}/{}",
                        destination,
                        get_resource_name(&a.clone().url).unwrap().as_str()
                    )
                    .as_str(),
                )
                .with_size(a.clone().size));
        }
    }
    Err("Artifact not found".to_string())
}

fn get_natives_value(n: &Option<LibraryNatives>) -> String {
    if let Some(n) = n {
        let os = OperatingSystem::detect();
        match os {
            OperatingSystem::Windows => {
                if let Some(raw) = &n.clone().windows {
                    fill(raw, "arch".to_string(), "x64".to_string()).to_string()
                } else {
                    "".to_string()
                }
            }
            OperatingSystem::Linux => {
                if let Some(raw) = &n.clone().linux {
                    fill(raw, "arch".to_string(), "x64".to_string()).to_string()
                } else {
                    "".to_string()
                }
            }
            _ => "".to_string(),
        }
    } else {
        "n".to_string()
    }
}
fn fill(s: &String, k: String, v: String) -> String {
    if !s.contains(k.as_str()) {
        return s.to_string();
    }
    let ss = s.replace(format!("${k}").as_str(), v.as_str());
    ss.clone()
}

fn find_out_os(rules: &[LibraryRule]) -> bool {
    let sys = OperatingSystem::detect();
    debug!("Finding out OS... {:?}", sys);
    for rule in rules {
        debug!("check... {:?}", rule);
        if !rule.allow(&sys) {
            return false;
        }
    }
    true
}
