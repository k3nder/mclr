
use std::cmp::PartialEq;
use std::path::Path;
use crate::deserialize::json_version::{Library, LibraryDownloads, LibraryNatives, LibraryRule};

use crate::utils::{CounterEvent, HandleEvent, io_utils};
use crate::utils::io_utils::{calc_sha1, download, get_resource_name, verify_size};
use crate::utils::io_utils::compress::extract_zip;
use crate::utils::io_utils::system::OperatingSystem;


struct MavenLibrary {
    pub groupID: String,
    pub artifactID: String,
    pub version: String,
    pub repository: String
}

impl MavenLibrary {
    pub fn parse(name: String, repository: String) -> Self {

        let tokens: Vec<&str> = name.split(":").collect();

        MavenLibrary {
            repository,
            groupID: tokens.get(0).unwrap().to_string(),
            artifactID: tokens.get(1).unwrap().to_string(),
            version: tokens.get(2).unwrap().to_string()
        }
    }

    pub fn all_URL(&self) -> String {
        let group = self.groupID.replace(".", "/");
        format!("{}{}/{}/{}/{}", self.repository, group, self.artifactID, self.version, self.cl_name())
    }

    pub fn cl_name(&self) -> String {
        format!("{}-{}.jar", self.artifactID, self.version)
    }
}

pub fn get_libs(destination: &str, binary_destination: &str, libs: &Vec<Library>, event: HandleEvent<CounterEvent>) -> Result<(), Box<dyn std::error::Error>> {
    let mut index = 0;
    for lib in libs {
        //println!("{}", &lib.clone().name.as_str());
        let natives = &&lib.clone().natives;
        if let Some(downloads) = &lib.clone().downloads {
            // artifact
            artifact_download(destination, &lib, &downloads);
            // classfiers
            classifier_download(destination, binary_destination, natives, &downloads);
        } else {
            let lib = MavenLibrary::parse(lib.clone().name, lib.clone().url);
            io_utils::download(format!("{}/{}", destination, lib.cl_name().as_str()).as_str(), lib.all_URL().as_str());
        }
        index += 1;
        event.event(CounterEvent::new(libs.len(), index));
    }

    Ok(())
}

pub fn check(destination: &str, binary_destination: &str, libs: &Vec<Library>, event: HandleEvent<CounterEvent>) -> bool {
    let mut index = 0;
    for lib in libs {
        //println!("{}", &lib.clone().name.as_str());
        let natives = &&lib.clone().natives;
        if let Some(downloads) = &lib.clone().downloads {
            // artifact
            artifact_check(destination, &lib, &downloads);
            // classfiers
            classifier_check(destination, natives, &downloads);
        } else {
            let mlib = MavenLibrary::parse(lib.clone().name, lib.clone().url);
            let file = format!("{}/{}", destination, mlib.cl_name().as_str());

            let _path = Path::new(&file);
            let _sha1 = calc_sha1(_path);

            return true;
        }
        index += 1;
        event.event(CounterEvent::new(libs.len(), index));
    }
    true
}

fn classifier_download(destination: &str, binary_destination: &str, natives: &&Option<LibraryNatives>, downloads: &&LibraryDownloads) {
    let clc = &downloads.clone().classifiers;
    if !clc.is_none() {
        let native_key = get_natives_value(natives.clone());
        if let Some(n) = &clc.clone().unwrap().get(&native_key) {

            let file = format!("{}/{}", destination, get_resource_name(&n.clone().url).unwrap().as_str());

            download(&file, &n.clone().url);
            extract_zip(binary_destination, file.as_str());
        }
    }
}



fn classifier_check(destination: &str, natives: &&Option<LibraryNatives>, downloads: &&LibraryDownloads) -> bool {
    let clc = &downloads.clone().classifiers;
    if !clc.is_none() {
        let native_key = get_natives_value(natives.clone());
        if let Some(n) = &clc.clone().unwrap().get(&native_key) {

            let file = format!("{}/{}", destination, get_resource_name(&n.clone().url).unwrap().as_str());

            let _path = Path::new(&file);
            let _sha1 = calc_sha1(_path);

            return !(!verify_size(_path, n.size) || !n.sha1.eq(&_sha1))
        }
    }
    true
}

fn artifact_download(destination: &str, lib: &&Library, downloads: &&LibraryDownloads) {
    if let Some(a) = &downloads.clone().artifact {
        let file = format!("{}/{}", destination, get_resource_name(&a.clone().url).unwrap().as_str());
        if let Some(r) = &lib.clone().rules {
            if find_out_os(r) {
                download(&file, &a.clone().url);
            }
        } else {
            download(format!("{}/{}", destination, get_resource_name(&a.clone().url).unwrap().as_str()).as_str(), &a.clone().url);
        }
    }
}


fn artifact_check(destination: &str, lib: &&Library, downloads: &&LibraryDownloads) -> bool {
    if let Some(a) = &downloads.clone().artifact {
        let file = format!("{}/{}", destination, get_resource_name(&a.clone().url).unwrap().as_str());
        if let Some(r) = &lib.clone().rules {
            if find_out_os(r) {

                let _path = Path::new(&file);
                let _sha1 = calc_sha1(_path);

                return !(!verify_size(_path, a.size) || !a.sha1.eq(&_sha1))
            }
        } else {
            let _path = Path::new(&file);
            let _sha1 = calc_sha1(_path);

            return !(!verify_size(_path, a.size) || !a.sha1.eq(&_sha1))
        }
    }
    true
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
            },
            OperatingSystem::Linux => {
                if let Some(raw) = &n.clone().linux {
                    fill(raw, "arch".to_string(), "x64".to_string()).to_string()
                } else {
                    "".to_string()
                }
            }
            _ => { "".to_string() }
        }
    } else {
        "n".to_string()
    }
}
fn fill(s: &String, k: String, v: String) -> String {
    if !s.contains(k.as_str()) { return s.to_string(); }
    let ss = s.replace(format!("${k}").as_str(), v.as_str());
    ss.clone()
}

fn find_out_os(rules: &[LibraryRule]) -> bool {
    let sys = OperatingSystem::detect();
    for rule in rules {
        if !rule.allow(&sys) { return false }
    }
    true
}