use libc as c;
use std::ffi::CStr;
use std::io;
use std::net::SocketAddr;
use std::str;

use addr::MySocketAddr;
use err::lookup_errno;

/// Retrieve the name for a given IP and Service. Acts as a thin wrapper around
/// the libc getnameinfo.
///
/// Returned names may be encoded in puny code for Interational Domain Names
/// (UTF8 DNS names). You can use the `idna` crate to decode these to their
/// actual UTF8 representation.
///
/// Retrieving names or services that contain non-UTF8 locales is currently not
/// supported (as String is returned). Raise an issue if this is a concern for
/// you.
pub fn getnameinfo(sock: &SocketAddr, flags: c::c_int) -> io::Result<(Option<String>, Option<String>)> {
  // Convert the socket into our type, so we can get a sockaddr_in{,6} ptr.
  let sock: MySocketAddr = sock.clone().into();
  let (c_sock, c_sock_len) = sock.into_inner();

  // Allocate buffers for name and service strings.
  let mut c_host = [0 as c::c_char; c::NI_MAXHOST as usize];
  // No NI_MAXSERV, so use suggested value.
  let mut c_service = [0 as c::c_char; 32 as usize];

  unsafe {
    let res = lookup_errno(
      c::getnameinfo(
        c_sock, c_sock_len,
        c_host.as_mut_ptr(),
        c_host.len() as u32,
        c_service.as_mut_ptr(),
        c_service.len() as u32,
        flags
      )
    );

    match res {
      Ok(_) => {},
      #[cfg(unix)]
      Err(e) => {
        // Add workaround for getaddrinfo bug, as it might affect getnameinfo
        // too. Refer to the getaddrinfo comment in this crate for details.
        c::res_init();
        return Err(e)
      },
      #[cfg(not(unix))]
      Err(e) => return Err(e),
    };
  };

  let host = unsafe {
    CStr::from_ptr(c_host.as_ptr())
  };
  let service = unsafe {
    CStr::from_ptr(c_service.as_ptr())
  };

  let host = match str::from_utf8(host.to_bytes()) {
    Ok(name) => Ok(name.to_owned()),
    Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                   "Host UTF8 parsing failed"))
  }?;

  let service = match str::from_utf8(service.to_bytes()) {
    Ok(service) => Ok(service.to_owned()),
    Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                   "Service UTF8 parsing failed"))
  }?;

  Ok((Some(host), Some(service)))
}

#[test]
fn test_getnameinfo() {
   use std::net::{IpAddr, SocketAddr};

   let ip: IpAddr = "127.0.0.1".parse().unwrap();
   let port = 22;
   let socket: SocketAddr = (ip, port).into();

   let (name, service) = match getnameinfo(&socket, 0) {
     Ok((n, s)) => (n, s),
     Err(e) => panic!("Failed to lookup socket {:?}", e),
   };

   assert_eq!(name.unwrap(), "localhost");
   assert_eq!(service.unwrap(), "ssh");
}
