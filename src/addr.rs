use libc as c;
use std::io;
use std::mem;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Copy, Clone)]
pub enum MySocketAddr {
  V4(MySocketAddrV4),
  V6(MySocketAddrV6),
}

impl MySocketAddr {
  pub fn into_inner(&self) -> (*const c::sockaddr, c::socklen_t) {
    match *self {
      MySocketAddr::V4(ref s) =>
        s.into_inner(),
      MySocketAddr::V6(ref s) =>
        s.into_inner()
    }
  }

  pub fn from_inner(sock: *mut c::sockaddr, sock_len: c::socklen_t) -> io::Result<Self> {
    // let family: c::sa_family_t = unsafe {
    //   match storage.as_ref() {
    //     Some(s) => (*s).ss_family,
    //     None => return Err(io::Error::new(io::ErrorKind::Other, "Socket address null")),
    //   }
    // };
    let family;
    unsafe {
      family = (*sock).sa_family;
    }

    match family as c::c_int {
      c::AF_INET => {
        assert!(sock_len as usize >= mem::size_of::<c::sockaddr_in>());
        Ok(
          MySocketAddr::V4(
            MySocketAddrV4
              ::from(unsafe { *(sock as *const _ as *const c::sockaddr_in) })
          )
        )
      }
      c::AF_INET6 => {
        assert!(sock_len as usize >= mem::size_of::<c::sockaddr_in6>());
        Ok(
          MySocketAddr::V6(
            MySocketAddrV6
              ::from(unsafe { *(sock as *const _ as *const c::sockaddr_in6) })
          )
        )
      }
      _ => {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid argument"))
      }
    }
  }
}

impl From<SocketAddr> for MySocketAddr {
  fn from(sock: SocketAddr) -> MySocketAddr {
    match sock {
      SocketAddr::V4(s) => MySocketAddr::V4(s.into()),
      SocketAddr::V6(s) => MySocketAddr::V6(s.into()),
    }
  }
}

impl From<MySocketAddr> for SocketAddr {
  fn from(sock: MySocketAddr) -> SocketAddr {
    match sock {
      MySocketAddr::V4(s) => SocketAddr::V4(s.into()),
      MySocketAddr::V6(s) => SocketAddr::V6(s.into()),
    }
  }
}

#[derive(Copy, Clone)]
/// A wrapper around a libc sockaddr_in.
pub struct MySocketAddrV4 {
  inner: c::sockaddr_in
}

impl MySocketAddrV4 {
  fn new(ip: &Ipv4Addr, port: u16) -> MySocketAddrV4 {
    MySocketAddrV4 {
      inner: c::sockaddr_in {
        sin_family: c::AF_INET as c::sa_family_t,
        sin_port: port.to_be(),
        sin_addr: ipv4_to_inner(ip),
        .. unsafe { mem::zeroed() }
      }
    }
  }

  fn ip(&self) -> &Ipv4Addr {
      unsafe {
          &*(&self.inner.sin_addr as *const c::in_addr as *const Ipv4Addr)
      }
  }

  fn port (&self) -> u16 { self.inner.sin_port
  }

  fn into_inner(&self) -> (*const c::sockaddr, c::socklen_t) {
    (&self.inner as *const c::sockaddr_in as  *const c::sockaddr,
     mem::size_of_val(&self.inner) as c::socklen_t)
  }
}

impl From<c::sockaddr_in> for MySocketAddrV4 {
  fn from(sock: c::sockaddr_in) -> Self {
    MySocketAddrV4 { inner: sock }
  }
}

impl From<MySocketAddrV4> for SocketAddrV4 {
  fn from(sock: MySocketAddrV4) -> SocketAddrV4 {
    SocketAddrV4::new(sock.ip().clone(), sock.port())
  }
}

impl From<SocketAddrV4> for MySocketAddrV4 {
  fn from(sock: SocketAddrV4) -> MySocketAddrV4 {
    MySocketAddrV4::new(sock.ip(), sock.port())
  }
}

#[derive(Copy, Clone)]
/// A wrapper around a libc sockaddr_in6.
pub struct MySocketAddrV6 {
  inner: c::sockaddr_in6
}

impl MySocketAddrV6 {
  fn new(ip: &Ipv6Addr, port: u16, flowinfo: u32, scope_id: u32) -> MySocketAddrV6 {
    MySocketAddrV6 {
      inner: c::sockaddr_in6 {
        sin6_family: c::AF_INET6 as c::sa_family_t,
        sin6_port: port.to_be(),
        sin6_addr: ipv6_to_inner(ip),
        sin6_flowinfo: flowinfo,
        sin6_scope_id: scope_id,
        .. unsafe { mem::zeroed() }
      }
    }
  }

  fn ip(&self) -> &Ipv6Addr {
      unsafe {
          &*(&self.inner.sin6_addr as *const c::in6_addr as *const Ipv6Addr)
      }
  }

  fn port (&self) -> u16 {
    self.inner.sin6_port
  }

  fn into_inner(&self) -> (*const c::sockaddr, c::socklen_t) {
    (&self.inner as *const c::sockaddr_in6 as *const c::sockaddr,
     mem::size_of_val(&self.inner) as c::socklen_t)
  }
}

impl From<c::sockaddr_in6> for MySocketAddrV6 {
  fn from(sock: c::sockaddr_in6) -> Self {
    MySocketAddrV6 { inner: sock }
  }
}

impl From<MySocketAddrV6> for SocketAddrV6 {
  fn from(sock: MySocketAddrV6) -> SocketAddrV6 {
    SocketAddrV6::new(sock.ip().clone(), sock.port(), 0, 0)
  }
}

impl From<SocketAddrV6> for MySocketAddrV6 {
  fn from(sock: SocketAddrV6) -> MySocketAddrV6 {
    MySocketAddrV6::new(sock.ip(), sock.port(), 0, 0)
  }
}

/// Change an Ipv4Addr into a libc in_addr.
fn ipv4_to_inner(ip: &Ipv4Addr) -> c::in_addr {
  let o = ip.octets();
  c::in_addr {
    s_addr: (
      ((o[0] as u32) << 24) |
      ((o[1] as u32) << 16) |
      ((o[2] as u32) <<  8) |
      (o[3] as u32)
    ).to_be(),
  }
}

/// Change an Ipv6Addr into a libc in6_addr.
fn ipv6_to_inner(ip: &Ipv6Addr) -> c::in6_addr {
  let o = ip.octets();
  let mut addr: c::in6_addr = unsafe { mem::zeroed() };
  addr.s6_addr = o;
  addr
}

/// Turn an IpAddr into a libc (socketaddr, length) pair.
pub fn ip_to_sockaddr(ip: &IpAddr) ->  MySocketAddr {
  match ip {
    &IpAddr::V4(ipv4) => {
      MySocketAddr::V4(MySocketAddrV4::new(&ipv4, 0))
    },
    &IpAddr::V6(ipv6) => {
      MySocketAddr::V6(MySocketAddrV6::new(&ipv6, 0, 0, 0))
    },
  }
}

#[test]
fn test_ipv4_conversion() {
  use std::net::Ipv4Addr;
  let ips: Vec<Ipv4Addr> =
    ["127.0.0.1", "8.8.8.8", "172.16.0.1", "192.168.0.1"].iter()
    .map(|a| a.parse().unwrap())
    .collect();
  let converted: Vec<_> = ips.iter()
    .map(|ip| {
      let in_addr = ipv4_to_inner(ip);
      let ip = unsafe {
          &*(&in_addr as *const c::in_addr as *const Ipv4Addr)
      };
      ip.clone()
    }).collect();
  assert_eq!(ips, converted);
}

#[test]
fn test_ipv6_conversion() {
  use std::net::Ipv6Addr;
  let ips: Vec<Ipv6Addr> =
    ["::1", "fe80::1", "2017::DEAD:BEEF", "1234:5678:90ab:cdef:1357:9ace:2468:0bdf"].iter()
    .map(|a| a.parse().unwrap())
    .collect();
  let converted: Vec<_> = ips.iter()
    .map(|ip| {
      let in6_addr = ipv6_to_inner(ip);
      let ip = unsafe {
          &*(&in6_addr as *const c::in6_addr as *const Ipv6Addr)
      };
      ip.clone()
    }).collect();
  assert_eq!(ips, converted);
}

#[test]
fn test_ipv4_to_sockaddr() {
  use std::net::IpAddr;

  let ips: Vec<IpAddr> =
    ["127.0.0.1", "8.8.8.8", "172.16.0.1", "192.168.0.1"].iter()
    .map(|a| a.parse().unwrap())
    .collect();
  let sockets: Vec<_> = ips.iter().map(|a| (a, ip_to_sockaddr(a))).collect();

  for (ip, socket) in sockets {
    let socket = match socket {
      MySocketAddr::V4(s) => s,
      MySocketAddr::V6(_) =>
        panic!("Got Ipv6Addr, expected Ipv4Addr"),
    };
    assert_eq!(ip, socket.ip());
  }
}

#[test]
fn test_ipv6_to_sockadr() {
  use std::net::IpAddr;

  let ips: Vec<IpAddr> =
    ["::1", "fe80::1", "2017::DEAD:BEEF", "1234:5678:90ab:cdef:1357:9ace:2468:0bdf"].iter()
    .map(|a| a.parse().unwrap())
    .collect();
  let sockets: Vec<_> = ips.iter().map(|a| (a, ip_to_sockaddr(a))).collect();

  for (ip, socket) in sockets {
    let socket = match socket {
      MySocketAddr::V4(_) =>
        panic!("Got Ipv4Addr, expected Ipv6Addr"),
      MySocketAddr::V6(s) => s,
    };
    assert_eq!(ip, socket.ip());
  }
}
