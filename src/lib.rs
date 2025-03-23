pub mod deserialize;
pub mod mc;
pub mod utils;

mod tests {
    use std::{fs, path::Path};

    use crate::{
        mc::{
            self, get_compatible_java,
            utils::command_builder::{
                CommandAssetsConfig, CommandRamConfig, CommandResourcesConfig, CommandUserConfig,
                RunType,
            },
        },
        utils::{manifest::manifest, HandleEvent},
    };

    #[test]
    fn main() {
        if !Path::new("versions").exists() {
            fs::create_dir("versions").expect("Cannot create versions")
        }
        //CONSOLE_HISTORY.push("downloading...".to_string());
        let b = manifest();
        let versions = b.get("1.20.1").unwrap();

        if !Path::new(format!("versions/{}", versions.id).as_str()).exists() {
            fs::create_dir(format!("versions/{}", &versions.id))
                .expect("Cannot create versions dir")
        }
        let version = &versions
            .save_and_load(format!("versions/{}/{}.json", &versions.id, &versions.id).as_str());
        // json_version::load("versions/quilt-loader-0.26.3-1.21.1/quilt-loader-0.26.3-1.21.1.json");

        println!("{:?}", version);

        let java_home = get_compatible_java("dest", &version.java_version);
        // paths and parameters
        let jar_path = format!("versions/{}/{}.jar", &version.id, &version.id);
        let libs_path = format!("versions/{}/libraries", &version.id);
        let binary_path = format!("versions/{}/bin", &version.id);
        let libs = &version.clone().libraries;

        //while !mc::utils::libs_utils::verify(&*libs_path.clone(), json_version::load("versions/1.8.9/1.8.9.json").libraries) {
        mc::utils::libs_utils::filter_libs(
            &libs_path.clone(),
            binary_path.as_str(),
            &libs.clone(),
            HandleEvent::new(move |_| {
                //println!("{}", e.percent());
            }),
        )
        .expect("TODO: panic message")
        .start();

        //mc::utils::libs_utils::get_libs(&libs_path.clone(), binary_path.as_str(), &version.clone().libraries, HandleEvent::new(move |e| {}));
        //println!("{}", mc::utils::libs_utils::verify(&libs_path.clone(), libs.clone(), HandleEvent::new(move |e| {})));

        mc::download(&jar_path, &version);

        mc::utils::assets_utils::download_all(
            "assets",
            &version,
            HandleEvent::new(|_| {
                //println!("{}", e.percent())
            }),
        );
        println!("{}", jar_path.as_str());

        if let Some(logg) = &version.clone().logging {
            mc::get_config_logger(logg, "assets/log/log4j.xml");
        }

        println!("Running");

        mc::utils::command_builder::Command {
            resources: CommandResourcesConfig {
                libraries: libs_path.clone().to_string(),
                jar_file: jar_path.to_string(),
                bin: binary_path.to_string(),
                logger: "assets/log/log4j.xml".to_string(),
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
            ram: CommandRamConfig { xmx: 4, xms: 2 },
            event: |_s: String| {
                println!("{}", _s);
            },
            err_event: |e: String| {
                println!("{}", e);
            },
            args: vec![],
        }
        .run(RunType::SERVER("oblision.es".to_string()));
    }
}
