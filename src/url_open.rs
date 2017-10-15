//! # url_open
//! A simple crate to open URLs in the default web browser.
//!
//! ### Usage
//!
//! ```no_run
//! extern crate url;
//! extern crate url_open;
//!
//! use url::Url;
//! use url_open::UrlOpen;
//!
//! fn main() {
//!     Url::parse("https://github.com/overdrivenpotato/url_open").unwrap().open();
//! }
//! ```

extern crate url;

use url::Url;
use std;

/// Convenience method to open URLs
pub trait UrlOpen {
    fn open(&self);
}

impl UrlOpen for Url {
    fn open(&self) {
        open(self);
    }
}

#[cfg(target_os = "windows")]
pub fn open(url: &Url) {
    extern crate shell32;
    extern crate winapi;

    use std::ffi::CString;
    use std::ptr;

    unsafe {
        shell32::ShellExecuteA(
            ptr::null_mut(),
            CString::new("open").unwrap().as_ptr(),
            CString::new(url.to_string().replace("\n", "%0A"))
                .unwrap()
                .as_ptr(),
            ptr::null(),
            ptr::null(),
            winapi::SW_SHOWNORMAL,
        );
    }
}

#[cfg(target_os = "macos")]
pub fn open(url: &Url) {
    let _ = std::process::Command::new("open")
        .arg(url.to_string())
        .output();
}

#[cfg(target_os = "linux")]
pub fn open(url: &Url) {
    let _ = std::process::Command::new("xdg-open")
        .arg(url.to_string())
        .output();
}
