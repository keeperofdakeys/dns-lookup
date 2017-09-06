extern crate libc;
extern crate dns_lookup as dns;

use std::io;

fn main() {
  let mut hints = dns::AddrInfoHints::default();
  hints.flags = 0x0040;
  // hints.socktype = dns::SockType::Stream;
  // hints.address = dns::AddrFamily::Inet;
  // hints.protocol = dns::ProtoFamily::Inet;
  unsafe {
    let cstr = std::ffi::CString::new("").unwrap();
    libc::setlocale(libc::LC_ALL, cstr.as_ptr() as *const _ as *const i8);
  }
  let list: io::Result<Vec<_>> =
    dns::getaddrinfo(Some("☃.net"), Some("http"), Some(hints)).unwrap().collect();
  println!("{:?}", list);
}
