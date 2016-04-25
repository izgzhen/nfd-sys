#![crate_name = "nfd"]

extern crate libc;

#[cfg(target_os = "linux")]
extern crate gtk;

#[cfg(target_os = "macos")]
extern crate cocoa;

use libc::{size_t, c_char};

pub type NFDChar = c_char;

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

#[link(name = "nfd", kind = "static")]
extern {
    /* single file open dialog */
    pub fn NFD_OpenDialog(filterList: *const NFDChar,
                          defaultPath: *const NFDChar,
                          outPath: *mut (*mut NFDChar)) -> NFDResult;

    /* multiple file open dialog */
    pub fn NFD_OpenDialogMultiple(filterList: *const NFDChar,
                                  defaultPath: *const NFDChar,
                                  outPaths: *mut NFDPathSet) -> NFDResult;

    /* save dialog */
    pub fn NFD_SaveDialog(filterList: *const NFDChar,
                          defaultPath: *const NFDChar,
                          outPath: *mut (*mut NFDChar)) -> NFDResult;

    /* get last error -- set when nfdresult_t returns NFD_ERROR */
    pub fn NFD_GetError() -> *const NFDChar;

    /* get the number of entries stored in pathSet */
    pub fn NFD_PathSet_GetCount(pathSet: *const NFDPathSet) -> size_t;


    /* Get the UTF-8 path at offset index */
    pub fn NFD_PathSet_GetPath(pathSet: *const NFDPathSet,
                               index: size_t) -> *mut NFDChar;

    /* Free the pathSet */
    pub fn NFD_PathSet_Free(pathSet: *mut NFDPathSet);
}
