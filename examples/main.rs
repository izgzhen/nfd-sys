extern crate nfd;
extern crate libc;
use std::ptr;
use nfd::*;
use std::ffi::CString;

fn main() {
    unsafe {
        // let mut out_path = CString::new("").unwrap();
        let out_path: *mut (*mut NFDChar) = &mut ptr::null_mut();
        let result: NFDResult = NFD_OpenDialog(CString::new("png,jpg;pdf").unwrap().as_ptr(), ptr::null(), out_path);

        let cstring = CString::from_raw(*out_path);
        println!("Result: {:?}", result);
        println!("cstring: {:?}", cstring);
    }
}
