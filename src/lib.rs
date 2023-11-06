// TODO: documentation.

use libc::{c_char, c_void};
use std::ffi::{CStr, CString};

#[repr(C)]
// TODO: documentation.
struct PkrsPackage {
    id: *const c_char,
    name: *const c_char,
    version: *const c_char,
    summary: *const c_char,
    arch: *const c_char,
    data: *const c_char,
}

// TODO: documentation.
pub struct Package {
    pub id: String,
    pub name: String,
    pub version: String,
    pub summary: String,
    pub arch: String,
    pub data: String,
}

#[link(name = "pk_rs")]
extern "C" {
    fn pkrs_get_package_details(package_name: *const c_char) -> *mut *mut PkrsPackage;
}

// TODO: better error handling.
// TODO: documentation.
pub fn get_package_details(package_name: &str) -> Vec<Package> {
    let mut packages = Vec::new();

    unsafe {
        let package_name = CString::new(package_name).unwrap();
        let res = pkrs_get_package_details(package_name.as_ptr());

        if !res.is_null() {
            let mut index = 0;

            while !(*res.offset(index)).is_null() {
                let package = *res.offset(index);
                packages.push(Package {
                    id: CStr::from_ptr((*package).name)
                        .to_string_lossy()
                        .to_string(),
                    name: CStr::from_ptr((*package).name)
                        .to_string_lossy()
                        .to_string(),
                    version: CStr::from_ptr((*package).name)
                        .to_string_lossy()
                        .to_string(),
                    summary: CStr::from_ptr((*package).name)
                        .to_string_lossy()
                        .to_string(),
                    arch: CStr::from_ptr((*package).name)
                        .to_string_lossy()
                        .to_string(),
                    data: CStr::from_ptr((*package).name)
                        .to_string_lossy()
                        .to_string(),
                });

                index += 1;
            }
        }

        libc::free(res as *mut c_void);
    }

    packages
}
