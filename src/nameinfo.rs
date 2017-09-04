use libc as c;
use std::ffi::CStr;
use std::io;
use std::net::SocketAddr;
use std::str;

use addr::MySocketAddr;
use err::lookup_errno;

pub fn getnameinfo(sock: &SocketAddr, flags: c::c_int) -> io::Result<(Option<String>, Option<String>)> {
  // Convert the socket into our type, so we can get a sockaddr_in{,6} ptr.
  let sock: MySocketAddr = sock.clone().into();
  let (c_sock, c_sock_len) = sock.into_inner();

  // Allocate buffers for name and service strings.
  let mut c_host = [0 as c::c_char; c::NI_MAXHOST as usize];
  // No NI_MAXSERV, so use suggested value.
  let mut c_service = [0 as c::c_char; 32 as usize];

  unsafe {
    lookup_errno(
      c::getnameinfo(
        c_sock, c_sock_len,
        c_host.as_mut_ptr(),
        c_host.len() as u32,
        c_service.as_mut_ptr(),
        c_service.len() as u32,
        flags
      )
    )?
  };

  let host = unsafe {
    CStr::from_ptr(c_host.as_ptr())
  };
  let service = unsafe {
    CStr::from_ptr(c_service.as_ptr())
  };

  // TODO: Should this be OsString, due to encoding issues?
  let host = match str::from_utf8(host.to_bytes()) {
    Ok(name) => Ok(name.to_owned()),
    Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                   "Host UTF8 parsing failed"))
  }?;
  // TODO: Should this be OsString, due to encoding issues?
  let service = match str::from_utf8(service.to_bytes()) {
    Ok(service) => Ok(service.to_owned()),
    Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                   "Service UTF8 parsing failed"))
  }?;

  Ok((Some(host), Some(service)))
}

#[test]
fn test_getnameinfo() {
  println!("{:?}", getnameinfo(&SocketAddr::new("127.0.0.1".parse().unwrap(), 53), 0));
}
