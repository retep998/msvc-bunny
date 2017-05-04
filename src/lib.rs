// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
#![allow(bad_style)]
#[macro_use] extern crate winapi;
pub mod setup_config;
pub mod util;
mod registry;

use std::env::var_os;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub struct Toolchain {
    env_path: Vec<PathBuf>,
    env_lib: Vec<PathBuf>,
    env_include: Vec<PathBuf>,
    link: PathBuf,
    rc: PathBuf,
    cl: PathBuf,
}
impl Toolchain {
    // Path to cl.exe
    pub fn cl(&self) -> &Path {
        &self.cl
    }
    // Path to link.exe
    pub fn link(&self) -> &Path {
        &self.link
    }
    // Path to rc.exe
    pub fn rc(&self) -> &Path {
        &self.rc
    }
    // Paths to append to PATH.
    pub fn env_path(&self) -> &[PathBuf] {
        &self.env_path
    }
    // Paths to append to LIB.
    pub fn env_lib(&self) -> &[PathBuf] {
        &self.env_lib
    }
    // Paths to append to INCLUDE.
    pub fn env_include(&self) -> &[PathBuf] {
        &self.env_include
    }
}
struct Ucrt {
    include: PathBuf,
    lib: PathBuf,
}
impl Ucrt {
    fn find(arch: Arch, err: &mut Vec<Error>) -> Option<Ucrt> {
        let root = match find_kitroot("KitsRoot10") {
            Some(x) => x,
            None => return None,
        };
        let readdir = match read_dir(root.join("lib")) {
            Ok(x) => x,
            Err(_) => {
                err.push(Error::KitRootMissing);
                return None;
            },
        };
        // TODO
        // Iterate over 10.* directories
        // Check to make sure they have ucrt.lib
        unimplemented!()
    }
}
enum Arch {
    Arm,
    Arm64,
    X86,
    X64,
}
#[derive(Clone, Copy, Debug)]
pub enum Error {
    NoWindowsSDK,
    NoUniversalCRT,
    NoVCToolchain,
    KitRootMissing,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        use Error::*;
        let msg = match *self {
            NoWindowsSDK => "Failed to locate a suitable Windows SDK installation",
            NoUniversalCRT => "Detected VC++ toolchain requires the Universal CRT which could not be located",
            NoVCToolchain => "Failed to locate any suitable VC++ toolchain",
            KitRootMissing => "KitRoot found in registry but directory contains nothing",
        };
        f.write_str(msg)
    }
}

pub fn find_msvc_toolchain() -> (Option<Toolchain>, Vec<Error>) {
    let mut err = Vec::new();
    if var_os("VCINSTALLDIR").is_some() {
        let toolchain = find_preconfigured(&mut err);
        return (toolchain, err);
    }
    println!("{:?}", find_kitroot("KitsRoot10"));
    unimplemented!()
}
fn find_preconfigured(err: &mut Vec<Error>) -> Option<Toolchain> {
    // TODO
    // Find link.exe in PATH and make sure cl.exe is next to it
    // If link.exe can only be found without cl.exe emit a warning
    unimplemented!()
}
fn find_kitroot(name: &str) -> Option<PathBuf> {
    registry::LOCAL_MACHINE
        .open(r"SOFTWARE\Microsoft\Windows Kits\Installed Roots".as_ref())
        .and_then(|key| key.query_str(name)).ok()
        .map(|path| path.into())
}
