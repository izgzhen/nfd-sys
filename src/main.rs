extern crate nfd;
extern crate libc;
use libc::{size_t, c_char};
use std::ptr;
use nfd::*;
use std::ffi::CString;

fn main() {
    unsafe {
        let mut outPath = CString::new("aosidaoweaksnas").unwrap();
        let result: NFDResult = NFD_OpenDialog(CString::new("png,jpg;pdf").unwrap().as_ptr(), ptr::null(), &mut outPath.into_raw());

        println!("Result: {:?}", result);
    }
}
