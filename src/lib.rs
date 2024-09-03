
pub mod deserialize;
pub mod mc;
pub mod utils;
mod tests {
    use std::fmt::Write;
    use std::fs;
    use std::path::Path;
    use crate::deserialize::json_version;
    use crate::deserialize::json_version::JsonVersion;
    use crate::mc;
    use crate::mc::get_compatible_java;
    use crate::mc::utils::command_builder::{CommandAssetsConfig, CommandRamConfig, CommandResourcesConfig, CommandUserConfig};
    use crate::utils::HandleEvent;
    use crate::utils::manifest::manifest;
    use crate::utils::sync_utils::sync;
    #[test]
    fn main() {
        let version_index = Some(0);
        if !Path::new("versions").exists() { fs::create_dir("versions").expect("Cannot create versions") }
        if let Some(version_index) = version_index {
            //CONSOLE_HISTORY.push("downloading...".to_string());
            let b = manifest();
            let versions = b.versions.get(0).unwrap();


            if !Path::new(format!("versions/{}", versions.clone().id).as_str()).exists() { fs::create_dir(format!("versions/{}", &versions.clone().id)).expect("Cannot create versions dir") }
            let version = &versions.save_and_load(format!("versions/{}/{}.json", &versions.clone().id, &versions.clone().id).as_str());
                // json_version::load("versions/quilt-loader-0.26.3-1.21.1/quilt-loader-0.26.3-1.21.1.json");

            println!("{:?}", version);

            let java_home = get_compatible_java("dest", &version.javaVersion);
            // paths and parameters
            let jar_path = format!("versions/{}/{}.jar", &version.id, &version.id);
            let libs_path = format!("versions/{}/libraries", &version.id);
            let binary_path = format!("versions/{}/bin", &version.id);
            let libs = &version.clone().libraries;

            //while !mc::utils::libs_utils::verify(&*libs_path.clone(), json_version::load("versions/1.8.9/1.8.9.json").libraries) {
            mc::utils::libs_utils::get_libs(&libs_path.clone(), binary_path.as_str(), &libs.clone(), HandleEvent::new(move |e| {
                //println!("{}", e.percent());
            })).expect("TODO: panic message");

            //mc::utils::libs_utils::get_libs(&libs_path.clone(), binary_path.as_str(), &version.clone().libraries, HandleEvent::new(move |e| {}));
            //println!("{}", mc::utils::libs_utils::verify(&libs_path.clone(), libs.clone(), HandleEvent::new(move |e| {})));


            mc::download(&jar_path, &version);

            //mc::utils::assets_utils::download_all("assets", &version, HandleEvent::new(move |e| {
            //    //println!("{}", e);
            //}), HandleEvent::new(|e| {
            //    //println!("{}", e.percent())
            //}));
            //println!("{}", &java_home.clone().as_str());
            println!("{}", jar_path.as_str().clone());

            if let Some(logg) = &version.clone().logging {
                mc::get_config_logger(logg, "assets/log/log4j.xml");
            }

            mc::utils::command_builder::Command {
                resources: CommandResourcesConfig {
                    libraries: libs_path.clone().to_string(),
                    jar_file: jar_path.to_string(),
                    bin: binary_path.to_string(),
                    logger: "assets/log/log4j.xml".to_string()
                },
                java_home: java_home.to_string(),
                game_dir: ".min".to_string(),
                assets: CommandAssetsConfig {
                    assets_dir: "assets/".to_string(),
                    assets_index: version.assets.clone().to_string(),
                },
                user: CommandUserConfig {
                    user_type: "user".to_string(),
                    client_id: "0".to_string(),
                    uuid: "d0db8a3d-c392-4ae7-96e5-9365de33ab52".to_string(),
                    xuid: "0".to_string(),
                    access_token: "0".to_string(),
                    user_name: "tuser".to_string(),
                },
                version: version.command_conf(),
                ram: CommandRamConfig {
                    xmx: 4,
                    xms: 2,
                },
                event: |_s: String| {
                    println!("{}", _s);
                },
            }.run();
        }
//assert_eq!(true, true)
    }
}