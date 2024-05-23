use std::any::Any;
use std::cmp::PartialEq;
use std::fmt::format;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::ptr::null;
use tokio::io::join;
use crate::deserialize::json_version::{Library, LibraryDownloads, LibraryDownloadsArtifacts, LibraryDownloadsClassifiers, LibraryRule, LibraryRuleOs};
use crate::mc;
use crate::utils::{CounterEvent, HandleEvent, io_utils};
use crate::utils::io_utils::get_resource_name;
use crate::utils::io_utils::system::OperatingSystem;

pub fn get_libs(destination: &str, binary_destination: &str, libs: &Vec<Library>, event: HandleEvent<CounterEvent>) -> Result<(), Box<dyn std::error::Error>> {
    let mut index = 0;
    for lib in libs {
        let downloads = match &lib.downloads {
            Some(d) => (false, d),  // Si hay descargas, no es None
            None => (true, &LibraryDownloads { artifact: None, classifiers: None }),  // Usa un valor por defecto si no hay descargas
        };

        if downloads.0 {
            get_customs_libs(&lib, destination)?;
            continue;
        }

        if is_allowed_library(&lib) {
            //println!("downloading... {}", &lib.name);
            let lib_name = download(destination, downloads.1);
            if !&lib.extract.is_none() {
                io_utils::compress::extract_zip(binary_destination, format!("{}\\{}", destination, lib_name).as_str());
            }
        }
        index += 1;
        (event.event)(CounterEvent::new(libs.len(), index))
    }

    Ok(())
}


fn is_allowed_library(lib: &Library) -> bool {
    let clone = lib.clone();
    let cd = &clone.downloads;
    if cd.is_none() {
        return true;
    }
    let mut allow_download: bool = false;
    if let Some(rules) = &lib.rules {
        allow_download = find_out_os(rules);
    } else if let Some(downloads) = &lib.downloads {
        allow_download = comprove_classifiers(downloads);
        //println!("{}", allow_download);
    }
    allow_download
}

fn get_customs_libs(library: &Library, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let index_of_p = library.name.rfind(':').unwrap();
    let subversion = &library.name[(index_of_p + 1)..];
    let etc = &library.name[..index_of_p];
    let index_of = library.name.find(':').unwrap();
    let nana = &library.name[(index_of + 1)..index_of_p];

    let url = format!("{}{}{}{}{}{}{}{}{}", library.url, etc.replace(":", "/").replace(".", "/"), "/", subversion, "/", nana, "-", subversion, ".jar");
    let file_name = format!("{}{}", nana, ".jar");

    io_utils::download(format!("{}/{}", destination, file_name).as_str(), &*url);

    Ok(())
}

fn find_out_os(rules: &[LibraryRule]) -> bool {
    for rule in rules {
        if rule.action.eq("allow") {
            if rule.os.name.is_empty() || cfg!(target_os = "windows") == (rule.os.name == "windows") {
                return true;
            }
        } else if !rule.os.name.is_empty() && cfg!(target_os = "windows") != (rule.os.name == "windows") {
            return true;
        }
    }
    false
}

fn comprove_classifiers(downloads: &LibraryDownloads) -> bool {
    if downloads.classifiers.is_none() {
        return true;
    }
    let classifiers = downloads.classifiers.as_ref().unwrap();
    let mut result = false;
    if let Some(ref windows) = classifiers.windows {
        result = is_os(result, windows);
    }
    if let Some(ref windows64) = classifiers.windows64 {
        result = is_os(result, windows64);
    }
    if let Some(ref windows32) = classifiers.windows32 {
        result = is_os(result, windows32);
    }
    if let Some(ref nwin) = classifiers.natives_windows {
        //println!("natives");
        result = true;
    }
    result
}


fn is_os(def: bool, x: &LibraryDownloadsArtifacts) -> bool {
    let target_os = OperatingSystem::detect();
    //println!("{:?}", target_os);
    return match target_os {
        OperatingSystem::Windows => {
            true
        }
        OperatingSystem::Linux => {
            true
        }
        OperatingSystem::MacOS => {
            true
        }
        OperatingSystem::Other => {
            def
        }
    };
}

fn download(destination: &str, lib: &LibraryDownloads) -> String {
    let mut file_name: String = "lib.jar".to_string();
    if let Some(ref artifact) = lib.artifact {
        let url = &artifact.url;
        io_utils::download(format!("{}/{}", destination, io_utils::get_resource_name(url).expect("lib1.jar")).as_str(), url);
        file_name = get_resource_name(url).expect("e");
    } else if let Some(ref classifiers) = lib.classifiers {
        let (url, path) = if let Some(ref windows) = classifiers.windows {
            (&windows.url, &windows.path)
        } else if let Some(ref windows64) = classifiers.windows64 {
            (&windows64.url, &windows64.path)
        } else if let Some(ref nwin) = classifiers.natives_windows {
            (&nwin.url, &nwin.path)
        } else {
            return "e".to_string();
        };
        let mut dest_file = format!("{}/{}", destination, io_utils::get_resource_name(url).expect("lib.jar"));
        io_utils::download(dest_file.as_str(), url);
        file_name = get_resource_name(url).expect("a");
    }
    return file_name;
}

