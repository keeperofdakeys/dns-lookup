extern crate libc;
extern crate dns_lookup as dns;

use std::io;

fn main() {
  let mut hints = dns::AddrInfoHints::default();
  // hints.flags = 0x0040;
  let list: io::Result<Vec<_>> =
    dns::getaddrinfo(Some("☃.net"), Some("http"), Some(hints)).unwrap().collect();
  println!("{:?}", list);
}
