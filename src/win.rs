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
