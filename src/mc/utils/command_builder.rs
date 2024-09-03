use std::io::{BufRead, BufReader};
use std::process::Stdio;
use crate::utils::io_utils::system::OperatingSystem;

pub struct Command {
    pub resources: CommandResourcesConfig,
    pub java_home: String,
    pub game_dir: String,
    pub assets: CommandAssetsConfig,
    pub user: CommandUserConfig,
    pub version: CommandVersionConfig,
    pub ram: CommandRamConfig,
    pub event: fn(String)
}
pub struct CommandResourcesConfig {
    pub libraries: String,
    pub jar_file: String,
    pub bin: String,
    pub logger: String
}
pub struct CommandRamConfig {
    pub xmx: i32,
    pub xms: i32
}
pub struct CommandAssetsConfig {
    pub assets_dir: String,
    pub assets_index: String
}
pub struct CommandVersionConfig {
    pub version_id: String,
    pub version_type: String,
    pub main_class: String
}
pub struct CommandUserConfig {
    pub user_type: String,
    pub client_id: String,
    pub uuid: String,
    pub xuid: String,
    pub access_token: String,
    pub user_name: String
}
impl Command {
    pub fn run(&self) {
        //println!("{}", self.java_home.clone());

        match OperatingSystem::detect() { OperatingSystem::Linux => {
            let chmod = std::process::Command::new("/bin/chmod")
                .arg("+x")
                .arg(self.java_home.clone().as_str())
                .spawn();

            chmod.unwrap().wait().unwrap();
            }
            _ => {}
        }

        let mut child = std::process::Command::new(self.java_home.as_str())
            .arg(format!("-Djna.tmpdir={}", self.resources.bin))
            .arg(format!("-Dio.netty.native.workdir={}", self.resources.bin))
            .arg(format!("-Djava.library.path={}", self.resources.bin))
            .arg(format!("-Dlog4j.configurationFile={}", self.resources.logger))
            .arg("-cp")
            .arg(format!("{}{}{}/*", self.resources.jar_file, (match OperatingSystem::detect() { OperatingSystem::Linux => ":", _ => ";" }), self.resources.libraries))
            .arg(self.version.main_class.as_str())
            .arg("--version")
            .arg(self.version.version_id.as_str())
            .arg("--versionType")
            .arg(self.version.version_type.as_str())
            .arg("--accessToken")
            .arg(self.user.access_token.as_str())
            .arg("--uuid")
            .arg(self.user.uuid.as_str())
            .arg("--xuid")
            .arg(self.user.xuid.as_str())
            .arg("--clientId")
            .arg(self.user.client_id.as_str())
            .arg("--username")
            .arg(self.user.user_name.as_str())
            .arg("--userType")
            .arg(self.user.user_type.as_str())
            .arg("--assetIndex")
            .arg(self.assets.assets_index.as_str())
            .arg("--assetsDir")
            .arg(self.assets.assets_dir.as_str())
            .arg("--gameDir")
            .arg(self.game_dir.as_str())
            .stdout(Stdio::piped())
            .spawn().unwrap();
        //println!("run");
        // Obtener el stdout del proceso hijo
        let stdout = child.stdout.take().expect("Failed to capture stdout");

        // Leer la salida del proceso hijo de manera asíncrona
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            (self.event)(line.unwrap())
        }

        // Esperar a que el proceso hijo termine
        child.wait().unwrap();
    }
}