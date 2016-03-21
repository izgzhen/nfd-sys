#![crate_name = "nfd"]

extern crate libc;

#[cfg(target_os = "macos")]
extern crate cocoa;

use libc::{size_t, c_char};

#[repr(C)]
pub struct NFDPathSet {
    buf: *mut c_char,
    indices: *mut size_t, /* byte offsets into buf */
    count: size_t,        /* number of indices into buf */
}

#[repr(C)]
#[derive(Debug)]
pub enum NFDResult {
    NFD_ERROR,       /* programmatic error */
    NFD_OKAY,        /* user pressed okay, or successful return */
    NFD_CANCEL,      /* user pressed cancel */
}

#[link(name = "nfd")]
extern {
    pub fn NFD_OpenDialog(filterList: *const c_char,
                          defaultPath: *const c_char,
                          outPath: *mut (*mut c_char)) -> NFDResult;
}
