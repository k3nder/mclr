
use std::cmp::PartialEq;
use std::fmt::format;


use crate::deserialize::json_version::{Library, LibraryDownloads, LibraryNatives, LibraryRule};

use crate::utils::{CounterEvent, HandleEvent, io_utils};
use crate::utils::io_utils::{download, get_resource_name};
use crate::utils::io_utils::compress::{extract_zip, verify_integrity};
use crate::utils::io_utils::system::OperatingSystem;

pub fn get_libs(destination: &str, binary_destination: &str, libs: &Vec<Library>, event: HandleEvent<CounterEvent>) -> Result<(), Box<dyn std::error::Error>> {
    let mut index = 0;
    for lib in libs {
        println!("{}", &lib.clone().name.as_str());
        let natives = &&lib.clone().natives;
        if let Some(downloads) = &lib.clone().downloads {
            // artifact
            artifact_download(destination, &lib, &downloads);
            // classfiers
            classifier_download(destination, binary_destination, natives, &downloads);
        } else {
            // TODO custom download
            println!("download custom")
        }
        index += 1;
        event.event(CounterEvent::new(libs.len(), index));
    }

    Ok(())
}

fn classifier_download(destination: &str, binary_destination: &str, natives: &&Option<LibraryNatives>, downloads: &&LibraryDownloads) {
    if let Some(c) = &downloads.clone().classifiers {
        let native_key = get_natives_value(natives.clone());
        if let Some(n) = c.get(native_key.clone().as_str()) {
            // TODO download
            download(format!("{}/{}", destination, get_resource_name(&n.clone().url).unwrap().as_str()).as_str(), &n.clone().url);
            extract_zip(binary_destination, format!("{}/{}", destination, get_resource_name(&n.clone().url).unwrap().as_str()).as_str());
            println!("download key classfier")
        }
    }
}

fn artifact_download(destination: &str, lib: &&Library, downloads: &&LibraryDownloads) {
    if let Some(a) = &downloads.clone().artifact {
        if let Some(r) = &lib.clone().rules {
            if find_out_os(r) {
                download(format!("{}/{}", destination, get_resource_name(&a.clone().url).unwrap().as_str()).as_str(), &a.clone().url);
                println!("download rules artifact")
            }
        } else {
            // TODO download
            download(format!("{}/{}", destination, get_resource_name(&a.clone().url).unwrap().as_str()).as_str(), &a.clone().url);
            println!("download artifact")
        }
    }
}
fn artifact_verify(destination: &str, lib: &&Library, downloads: &&LibraryDownloads) -> bool {
    return if let Some(a) = &downloads.clone().artifact {
        if let Some(r) = &lib.clone().rules {
            if find_out_os(r) {
                //download(format!("{}/{}", destination, get_resource_name(&a.clone().url).unwrap().as_str()).as_str(), &a.clone().url);
                return verify_integrity(a.size, format!("{}\\{}",
                                                        destination,
                                                        get_resource_name(&a.clone().url).unwrap()
                ).as_str());
                //println!("download rules artifact")
            }
            false
        } else {
            // TODO download
            verify_integrity(a.size, format!("{}\\{}",
                                             destination,
                                             get_resource_name(&a.clone().url).unwrap()
            ).as_str())
            //println!("download artifact")
        }
    } else {
        false
    }
}

fn get_natives_value(n: &Option<LibraryNatives>) -> String {
    return if let Some(n) = n {
        let os = OperatingSystem::detect();
        match os {
            OperatingSystem::Windows => {
                let raw = &n.clone().windows;
                fill(raw, "arch".to_string(), "x64".to_string()).to_string()
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

pub fn verify(destination: &str, libs: &Vec<Library>, event: HandleEvent<CounterEvent>) -> bool {
    let mut index = 0;
    for lib in libs {
        println!("{}", &lib.clone().name.as_str());
        let natives = &&lib.clone().natives;
        if let Some(downloads) = &lib.clone().downloads {
            // artifact
            if !artifact_verify(destination, &lib, &downloads) { return false; }
            // classfiers
            //classifier_download(destination, binary_destination, natives, &downloads);
        } else {
            // TODO custom download
            println!("download custom")
        }
        index += 1;
        event.event(CounterEvent::new(libs.len(), index));
    }
    return true;
}
//fn is_allowed_library(lib: &Library) -> (bool, bool) {
//    let clone = lib;
//    let cd = &clone.downloads;
//    if cd.is_none() {
//        return (true, false);
//    }
//    let mut allow_download: (bool, bool) = (false, false);
//    if let Some(rules) = &lib.rules {
//        allow_download.0 = find_out_os(rules);
//    }
//    if let Some(downloads) = &lib.downloads {
//        allow_download.1 = find_out_classifiers(downloads);
//        //println!("{}", allow_download);
//    }
//    allow_download
//}

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

//fn find_out_classifiers(downloads: &LibraryDownloads) -> bool {
//    if downloads.classifiers.is_none() {
//        return true;
//    }
//    let classifiers = downloads.classifiers.as_ref().unwrap();
//    let mut result = false;
//    if let Some(ref _windows) = classifiers.windows {
//        result = is_os(result);
//    }
//    if let Some(ref _windows64) = classifiers.windows64 {
//        result = is_os(result);
//    }
//    if let Some(ref _windows32) = classifiers.windows32 {
//        result = is_os(result);
//    }
//    if let Some(ref _natives_win) = classifiers.natives_windows {
//        println!("natives");
//        result = is_os(result);
//    }
//    result
//}
//

fn is_os(def: bool) -> bool {
    let target_os = OperatingSystem::detect();
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

//fn download(destination: &str, lib: &LibraryDownloads, q: (bool, bool)) -> String {
//    let mut file_name: String = "lib.jar".to_string();
//    if let Some(ref artifact) = lib.artifact {
//        let url = &artifact.url;
//        if q.0 {io_utils::download(format!("{}/{}", destination, get_resource_name(url).expect("lib1.jar")).as_str(), url);}
//        println!("eee: {}", url.clone());
//        file_name = get_resource_name(url).expect("e");
//        println!("{}", &file_name.clone());
//    }
//    if let Some(ref classifiers) = lib.classifiers {
//        let (url, _path) = if let Some(ref windows) = classifiers.windows {
//            (&windows.url, &windows.path)
//        } else if let Some(ref windows64) = classifiers.windows64 {
//            (&windows64.url, &windows64.path)
//        } else if let Some(ref natives_win) = classifiers.natives_windows {
//            (&natives_win.url, &natives_win.path)
//        } else {
//            println!("eeeee");
//            return "e".to_string();
//        };
//        let dest_file = format!("{}/{}", destination, get_resource_name(url).expect("lib.jar"));
//        if q.1 {io_utils::download(dest_file.as_str(), url);}
//        file_name = get_resource_name(url).expect("a");
//    }
//    return file_name;
//}
//
//