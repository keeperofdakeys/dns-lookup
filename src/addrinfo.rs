#![allow(dead_code, unused)]

use libc as c;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::fmt;
use std::io;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
use std::ptr;

use addr::MySocketAddr;
use err::lookup_errno;
use types::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AddrInfo {
  pub flags: c::c_int,
  pub address: AddrFamily,
  pub socktype: SockType,
  pub protocol: ProtoFamily,
  pub sockaddr: SocketAddr,
  pub canonname: Option<String>,
}

impl AddrInfo {
  unsafe fn from_ptr<'a>(a: *mut c::addrinfo) -> io::Result<Self> {
    if a.is_null() {
      return Err(io::Error::new(io::ErrorKind::Other, "Supplied pointer is null."))?;
    }
    let addrinfo = *a;

    Ok(AddrInfo {
      flags: 0,
      address: AddrFamily::from_int(addrinfo.ai_family)
        .ok_or(
          io::Error::new(io::ErrorKind::Other,
          format!("Could not find family for: {}", addrinfo.ai_family))
        )?,
      socktype: SockType::from_int(addrinfo.ai_socktype)
        .ok_or(
          io::Error::new(io::ErrorKind::Other,
          format!("Could not find socket type for: {}", addrinfo.ai_socktype))
        )?,
      protocol: ProtoFamily::from_int(addrinfo.ai_protocol)
        .ok_or(
          io::Error::new(io::ErrorKind::Other,
          format!("Could not find protocol for: {}", addrinfo.ai_protocol))
        )?,
      sockaddr: MySocketAddr::from_inner(addrinfo.ai_addr, addrinfo.ai_addrlen)?.into(),
      canonname: addrinfo.ai_canonname.as_ref().map(|s|
        CStr::from_ptr(s).to_str().unwrap().to_owned()
      ),
    })
  }
}

pub struct AddrInfoIter {
  orig: *mut c::addrinfo,
  cur: *mut c::addrinfo,
}

impl Iterator for AddrInfoIter {
  type Item = io::Result<AddrInfo>;

  fn next(&mut self) -> Option<Self::Item> {
    unsafe {
      if self.cur.is_null() { return None; }
      let ret = AddrInfo::from_ptr(self.cur);
      self.cur = (*self.cur).ai_next as *mut c::addrinfo;
      Some(ret)
    }
  }
}

impl AddrInfoIter {
  /// Create an AddrInfo struct from a c addrinfo struct.
  fn new(host: &str) -> io::Result<Self> {
    let c_host = CString::new(host).unwrap();
    let mut res = ptr::null_mut();
    unsafe {
      c::getaddrinfo(c_host.as_ptr(), ptr::null(), ptr::null(), &mut res);
    }
    Ok(AddrInfoIter {
      orig: res,
      cur: res,
    })
  }
}

unsafe impl Sync for AddrInfoIter {}
unsafe impl Send for AddrInfoIter {}

impl Drop for AddrInfoIter {
    fn drop(&mut self) {
        unsafe { c::freeaddrinfo(self.orig) }
    }
}

pub fn getaddrinfo(host: Option<&str>, service: Option<&str>, hints: Option<&AddrInfo>)
    -> io::Result<AddrInfoIter> {
  // We must have at least host or service.
  if host.is_none() && service.is_none() {
    return Err(io::Error::new(io::ErrorKind::Other, "Either host or service must be supplied"));
  }

  if hints.is_some() {
    unimplemented!();
  }

  // Allocate CStrings, and keep around to free.
  let host = match host {
    Some(host_str) => Some(CString::new(host_str)?),
    None => None
  };
  let c_host = host.as_ref().map_or(ptr::null(), |s| s.as_ptr());
  let service = match service {
    Some(service_str) => Some(CString::new(service_str)?),
    None => None
  };
  let c_service = service.as_ref().map_or(ptr::null(), |s| s.as_ptr());

  let mut res = ptr::null_mut();
  unsafe {
    match lookup_errno(c::getaddrinfo(c_host, c_service, ptr::null(), &mut res)) {
      Ok(_) => {
        Ok(AddrInfoIter { orig: res, cur: res })
      },
      #[cfg(unix)]
      Err(e) => {
        c::res_init();
        Err(e)
      },
      #[cfg(not(unix))]
      Err(e) => Err(e),
    }
  }
}

#[test]
fn test_getaddrinfo() {
  for entry in getaddrinfo(Some("localhost"), None, None).unwrap() {
    println!("{:?}", entry);
  }
}
