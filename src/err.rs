use libc as c;
use std::ffi::{CStr};
use std::io;
use std::str;

#[cfg(all(not(windows), not(unix)))]
/// Given an errno, return an std::io::Result with the error message.
pub fn lookup_errno(err: c::c_int) -> io::Result<()> {
  match (err) {
    0 => Ok(()),
    _ => Err(io::Error::new(
      io::ErrorKind::Other,
       "failed to lookup address information"
    )),
  }
}

#[cfg(unix)]
/// Given an errno, return an std::io::Result with the error message.
pub fn lookup_errno(err: c::c_int) -> io::Result<()> {
  match err {
    0 => return Ok(()),
    c::EAI_SYSTEM => return Err(io::Error::last_os_error()),
    _ => {},
  }

  let detail = unsafe {
    str::from_utf8(CStr::from_ptr(c::gai_strerror(err)).to_bytes()).unwrap()
      .to_owned()
  };
  Err(io::Error::new(io::ErrorKind::Other,
    &format!("failed to lookup address information: {}", detail)[..]
  ))
}

#[cfg(windows)]
/// Given an errno, return an std::io::Result with the error message.
pub fn lookup_errno(err: c::c_int) -> io::Result<()> {
  match err {
    0 => Ok(()),
    _ => {
      io::Error::from_raw_os_error(
        unsafe { c::WASGetLastError() }
      )
    }
  }
}
