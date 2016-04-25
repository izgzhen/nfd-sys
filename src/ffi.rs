use libc::{size_t, c_char};

#[allow(non_camel_case_types)]
pub type nfdchar_t = c_char;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct nfdpathset_t {
    pub buf: *mut c_char,
    pub indices: *mut size_t, /* byte offsets into buf */
    pub count: size_t,        /* number of indices into buf */
}

#[repr(C)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum nfdresult_t {
    NFD_ERROR,       /* programmatic error */
    NFD_OKAY,        /* user pressed okay, or successful return */
    NFD_CANCEL,      /* user pressed cancel */
}

#[link(name = "nfd", kind = "static")]
extern {
    /* single file open dialog */
    pub fn NFD_OpenDialog(filterList: *const nfdchar_t,
                          defaultPath: *const nfdchar_t,
                          outPath: *mut (*mut nfdchar_t)) -> nfdresult_t;

    /* multiple file open dialog */
    pub fn NFD_OpenDialogMultiple(filterList: *const nfdchar_t,
                                  defaultPath: *const nfdchar_t,
                                  outPaths: *mut nfdpathset_t) -> nfdresult_t;

    /* save dialog */
    pub fn NFD_SaveDialog(filterList: *const nfdchar_t,
                          defaultPath: *const nfdchar_t,
                          outPath: *mut (*mut nfdchar_t)) -> nfdresult_t;

    /* get last error -- set when nfdresult_t returns NFD_ERROR */
    pub fn NFD_GetError() -> *const nfdchar_t;

    /* get the number of entries stored in pathSet */
    pub fn NFD_PathSet_GetCount(pathSet: *const nfdpathset_t) -> size_t;


    /* Get the UTF-8 path at offset index */
    pub fn NFD_PathSet_GetPath(pathSet: *const nfdpathset_t,
                               index: size_t) -> *mut nfdchar_t;

    /* Free the pathSet */
    pub fn NFD_PathSet_Free(pathSet: *mut nfdpathset_t);
}