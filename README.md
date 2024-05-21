```
    // ** use sync() to sync up the methods, manifest() to get the manifest and save_and_load() to save in a file the version and convert to readable object
    let version: json_version::JsonVersion = sync().block_on(manifest()).versions.first().unwrap().save_and_load("1.20.8.json");
    // ** get the java home path to use in command
    let java_home = get_compatible_java("desy", &version.javaVersion);
    // ** download jar in "v.jar" of version (&version)
    mc::mc::download("v.jar", &version);
    // ** download the assets in "assets" of the assets (version)
    mc::utils::assets_utils::download_all("assets", &version);
    // ** get all libs necessary to init the game
    mc::utils::libs_utils::get_libs("libs".as_ref(),"bin" , &version.libraries).expect("TODO: panic message");
    // ** execute the game
    mc::utils::command_builder::Command {
    resources: CommandResourcesConfig {
    libraries: "libs".to_string(),
    jar_file: "v.jar".to_string(),
    bin: "bin".to_string(),
    },
    java_home: java_home.to_string(),
    game_dir: "".to_string(),
    assets: CommandAssetsConfig {
    assets_dir: "assets\\".to_string(),
    assets_index: version.assets.to_string(),
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
    }.run();
```