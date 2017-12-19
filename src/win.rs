use std::ffi::CStr;
use std::net::UdpSocket;
use std::str;
use std::sync::{Once, ONCE_INIT};
use winapi::ctypes::c_char;
use winapi::um::winsock2::{WSAGetLastError,
                          gethostname as c_gethostname};

// Start windows socket library - From socket2-rs
pub(crate) fn init_winsock() {
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| {
        // Initialize winsock through the standard library by just creating a
        // dummy socket. Whether this is successful or not we drop the result as
        // libstd will be sure to have initialized winsock.
        let _ = UdpSocket::bind("127.0.0.1:34254");
    });
}

#[allow(dead_code)]
// Get hostname of local machine on windows.
// Used for reverse lookups of 127.0.0.1 tests on windows.
//
// Panics on failure.
pub(crate) fn get_hostname() -> String {
  init_winsock();

  let mut c_name = [0 as c_char; 256 as usize];
  let res = unsafe { c_gethostname(c_name.as_mut_ptr(), c_name.len() as i32) };
  if res != 0 {
    panic!("Error while calling gethostname: {}", unsafe { WSAGetLastError() });
  }

  let hostname = unsafe {
    CStr::from_ptr(c_name.as_ptr())
  };

  let hostname = str::from_utf8(hostname.to_bytes()).expect("UTF8 check failed").to_owned();

  return hostname;
}
