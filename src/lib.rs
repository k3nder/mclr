





pub mod deserialize;
pub mod mc;
pub mod utils;
#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use crate::mc::mc::get_compatible_java;
    use crate::utils::HandleEvent;
    use crate::utils::manifest::manifest;
    use crate::utils::sync_utils::sync;
    use super::*;
    #[test]
    fn main() {
        let version_index = Some(0);
        if !Path::new("versions").exists() { fs::create_dir("versions").expect("Cannot create versions") }
        if let Some(version_index) = version_index {
            //CONSOLE_HISTORY.push("downloading...".to_string());
            let versions = sync().block_on(manifest()).versions;
            let version_id = &versions.get(version_index).unwrap().id;
            if !Path::new(format!("versions/{version_id}").as_str()).exists() { fs::create_dir(format!("versions/{version_id}")).expect("Cannot create versions dir") }
            let version = &versions.get(version_index).unwrap().save_and_load(format!("versions/{}/{}.json", &version_id, &version_id).as_str());
            let java_home = get_compatible_java("dest", &version.javaVersion);
            // paths and parameters
            let jar_path = format!("versions/{}/{}.jar", &version.id, &version.id);
            let libs_path = format!("versions/{}/libraries", &version.id);
            let binary_path = format!("versions/{}/bin", &version.id);


            mc::mc::download(&jar_path, &version);
            if !mc::utils::assets_utils::verify("assets", &version, HandleEvent::new(move |e| {
                println!("{}", e.percent());
            })) {
                mc::utils::assets_utils::download_all("assets", &version, HandleEvent::new(move |e| {
                    println!("{}", e);
                }), HandleEvent::new(|e| {
                    println!("{}", e.percent())
                }));
            }
        } else {
            //CONSOLE_HISTORY.push("nothing selected".to_string());
        }
        assert_eq!(true, true)
    }
}