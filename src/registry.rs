// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::{OsString, OsStr};
use std::ptr::null_mut;
use winapi::shared::minwindef::{HKEY};
use winapi::um::winnt::{KEY_READ, KEY_WOW64_32KEY, REG_SZ};
use winapi::um::winreg::{
    HKEY_LOCAL_MACHINE, RRF_RT_REG_SZ, RegCloseKey, RegGetValueW, RegOpenKeyExW,
};

use util::{FromWide, ToWide};

struct OwnedKey(HKEY);
enum Repr {
    Const(HKEY),
    Owned(OwnedKey),
}
pub struct RegistryKey(Repr);
unsafe impl Sync for RegistryKey {}
unsafe impl Send for RegistryKey {}

pub static LOCAL_MACHINE: RegistryKey = RegistryKey(Repr::Const(HKEY_LOCAL_MACHINE));

impl RegistryKey {
    fn raw(&self) -> HKEY {
        match self.0 {
            Repr::Const(val) => val,
            Repr::Owned(ref val) => val.0,
        }
    }
    pub fn open(&self, key: &str) -> Result<RegistryKey, i32> {
        unsafe {
        let key = key.to_wide_null();
        let mut ret = null_mut();
        let err = RegOpenKeyExW(
            self.raw(), key.as_ptr(), 0, KEY_READ | KEY_WOW64_32KEY, &mut ret,
        );
        if err != 0 { return Err(err);}
        Ok(RegistryKey(Repr::Owned(OwnedKey(ret))))
        }
    }
    pub fn query_str(&self, name: &str) -> Result<OsString, i32> {
        unsafe {
            let name = name.to_wide_null();
            let mut len = 0;
            let err = RegGetValueW(
                self.raw(), null_mut(), name.as_ptr(), RRF_RT_REG_SZ,
                null_mut(), null_mut(), &mut len,
            );
            if err != 0 { return Err(err);}
            let mut v = vec![0; len as usize / 2];
            let err = RegGetValueW(
                self.raw(), null_mut(), name.as_ptr(), RRF_RT_REG_SZ,
                null_mut(), v.as_mut_ptr() as *mut _, &mut len,
            );
            if err != 0 { return Err(err);}
            Ok(OsString::from_wide_null(&v))
        }
    }
}
impl Drop for OwnedKey {
    fn drop(&mut self) {
        unsafe { RegCloseKey(self.0); }
    }
}
