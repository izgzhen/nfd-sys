extern crate nfd;
use nfd::*;

fn main() {
    match open_dialog_multiple(Some("png,jpg;pdf"), None) {
        Ok(paths) => println!("Paths: {:?}", paths),
        Err(e)    => println!("Errors: {:?}", e)
    }
}
