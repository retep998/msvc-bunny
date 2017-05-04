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

use std::env::var_os;
use std::fmt::{Display, Error as FmtError, Formatter};
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

#[derive(Clone, Copy, Debug)]
pub enum Error {
    NoWindowsSDK,
    NoUniversalCRT,
    NoVCToolchain,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        use Error::*;
        let msg = match *self {
            NoWindowsSDK => "Failed to locate a suitable Windows SDK installation",
            NoUniversalCRT => "Detected VC++ toolchain requires the Universal CRT which could not be located",
            NoVCToolchain => "Failed to locate any suitable VC++ toolchain",
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
    unimplemented!()
}
fn find_preconfigured(err: &mut Vec<Error>) -> Option<Toolchain> {
    // TODO
    // Find link.exe in PATH and make sure cl.exe is next to it
    // If link.exe can only be found without cl.exe emit a warning
    unimplemented!()
}
