use std::ffi::CStr;
use std::io;
use std::str;

/// Both libc and winapi define c_char as i8 `type c_char = i8;`
#[allow(non_camel_case_types)]
type c_char = i8;

#[cfg(unix)]
use libc::gethostname as c_gethostname;

/*
#[cfg(windows)]
use winapi::um::winsock2::gethostname as c_gethostname;
*/

#[cfg(windows)]
use windows_sys::Win32::Networking::WinSock::gethostname as c_gethostname;

/// Fetch the local hostname.
pub fn get_hostname() -> Result<String, io::Error> {
    // Prime windows.
    #[cfg(windows)]
    crate::win::init_winsock();

    let mut c_name = [0 as c_char; 256_usize];

    #[cfg(windows)]
    let res = unsafe { c_gethostname(c_name.as_mut_ptr() as *mut u8, c_name.len() as _) };

    #[cfg(unix)]
    let res = unsafe { c_gethostname(c_name.as_mut_ptr(), c_name.len() as _) };

    // If an error occured, check errno for error message.
    if res != 0 {
        return Err(io::Error::last_os_error());
    }

    let hostname = unsafe { CStr::from_ptr(c_name.as_ptr()) };

    str::from_utf8(hostname.to_bytes())
        .map(|h| h.to_owned())
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Non-UTF8 hostname"))
}

#[test]
fn test_get_hostname() {
    // We don't know the hostname of the local box, so just verify it doesn't return an error.
    get_hostname().unwrap();
}
