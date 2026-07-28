extern crate libc;
extern crate dns_lookup as dns;

use std::io;
use crate::dns::LookupErrorKind;

fn main() {
  let hints = dns::AddrInfoHints {
    flags: 0x0040,
    ..Default::default()
  };
  unsafe {
    let cstr = std::ffi::CString::new("").unwrap();
    libc::setlocale(libc::LC_ALL, cstr.as_ptr() as *const _);
  }
  let list: io::Result<Vec<_>> =
    dns::getaddrinfo(Some("☃.net"), Some("http"), Some(hints)).unwrap().collect();
  println!("{:?}", list);
  match dns::getaddrinfo(Some("☃.net"), Some("foobar"), Some(hints)) {
    Ok(_) => {},
    Err(e) => match e.kind() {
      LookupErrorKind::NoName => println!("NoName"),
      _ => println!("{:?}", e),
    }
  }
  let bar = dns::LookupError::new(0);
  println!("{:?} {:?}", bar, LookupErrorKind::new(0));
}
