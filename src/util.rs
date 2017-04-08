// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use std::ffi::OsString;
use std::mem::forget;
use std::ops::Deref;
use std::os::windows::ffi::OsStringExt;
use std::ptr::null_mut;
use std::slice::from_raw_parts;
use winapi::Interface;
use winapi::shared::wtypes::BSTR;
use winapi::um::combaseapi::CoInitializeEx;
use winapi::um::objbase::COINIT_MULTITHREADED;
use winapi::um::oleauto::{SysFreeString, SysStringLen};
use winapi::um::unknwnbase::IUnknown;
// COM must be initialized before you can use it
pub fn initialize_com() -> Result<i32, i32> {
    let err = unsafe { CoInitializeEx(null_mut(), COINIT_MULTITHREADED) };
    if err < 0 { return Err(err); }
    Ok(err)
}
// ComPtr to wrap COM interfaces sanely
pub struct ComPtr<T>(*mut T) where T: Interface;
impl<T> ComPtr<T> where T: Interface {
    /// Creates a `ComPtr` to wrap a raw pointer.
    /// It takes ownership over the pointer which means it does __not__ call `AddRef`.
    /// `T` __must__ be a COM interface that inherits from `IUnknown`.
    pub unsafe fn from_raw(ptr: *mut T) -> ComPtr<T> {
        assert!(!ptr.is_null());
        ComPtr(ptr)
    }
    /// Casts up the inheritance chain
    pub fn up<U>(self) -> ComPtr<U> where T: Deref<Target=U>, U: Interface {
        ComPtr(self.into_raw() as *mut U)
    }
    /// Extracts the raw pointer.
    /// You are now responsible for releasing it yourself.
    pub fn into_raw(self) -> *mut T {
        let p = self.0;
        forget(self);
        p
    }
    /// For internal use only.
    fn as_unknown(&self) -> &IUnknown {
        unsafe { &*(self.0 as *mut IUnknown) }
    }
    /// Performs QueryInterface fun.
    pub fn cast<U>(&self) -> Result<ComPtr<U>, i32> where U: Interface {
        let mut obj = null_mut();
        let err = unsafe { self.as_unknown().QueryInterface(&U::uuidof(), &mut obj) };
        if err < 0 { return Err(err); }
        Ok(unsafe { ComPtr::from_raw(obj as *mut U) })
    }
}
impl<T> Deref for ComPtr<T> where T: Interface {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.0 }
    }
}
impl<T> Clone for ComPtr<T> where T: Interface {
    fn clone(&self) -> Self {
        unsafe {
            self.as_unknown().AddRef();
            ComPtr::from_raw(self.0)
        }
    }
}
impl<T> Drop for ComPtr<T> where T: Interface {
    fn drop(&mut self) {
        unsafe { self.as_unknown().Release(); }
    }
}
pub struct BStr(BSTR);
impl BStr {
    pub unsafe fn from_raw(s: BSTR) -> BStr {
        BStr(s)
    }
    pub fn to_osstring(&self) -> OsString {
        let len = unsafe { SysStringLen(self.0) };
        let slice = unsafe { from_raw_parts(self.0, len as usize) };
        OsString::from_wide(slice)
    }
}
impl Drop for BStr {
    fn drop(&mut self) {
        unsafe { SysFreeString(self.0) };
    }
}