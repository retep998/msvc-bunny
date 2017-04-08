// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
extern crate msvc_bunny;
use msvc_bunny::setup_config::SetupConfiguration;
use msvc_bunny::util::initialize_com;
use std::fs::File;
use std::io::Read;
use std::path::{PathBuf};
// An example of using the safe wrapper
fn main() {
    initialize_com().unwrap();
    let config = SetupConfiguration::new().unwrap();
    let iter = config.enum_all_instances().unwrap();
    for instance in iter {
        let instance = instance.unwrap();
        let instance_path: PathBuf = instance.installation_path().unwrap().into();
        let version_path = instance_path.join(r"VC\Auxiliary\Build\Microsoft.VCToolsVersion.default.txt");
        let mut version_file = File::open(version_path).unwrap();
        let mut version = String::new();
        version_file.read_to_string(&mut version).unwrap();
        let host_arch = "x64";
        let target_arch = "x64";
        let vc_path = instance_path.join(r"VC\Tools\MSVC").join(version.trim()).join(r"bin").join(&format!("Host{}", host_arch)).join(&target_arch);
        let linker_path = vc_path.join("link.exe");
        if linker_path.is_file() {
            println!("Found linker at {}", linker_path.display());
        }
    }
}