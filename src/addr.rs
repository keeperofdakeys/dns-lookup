use libc as c;
use std::mem;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, IpAddr, Ipv4Addr, Ipv6Addr};

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

  fn port (&self) -> u16 {
    self.inner.sin_port
  }

  fn into_inner(self) -> (*const c::sockaddr, c::socklen_t) {
    (&self as *const _ as *const _, mem::size_of_val(&self) as c::socklen_t)
  }
}

impl From<c::sockaddr_in> for MySocketAddrV4 {
  fn from(sock: c::sockaddr_in) -> Self {
    MySocketAddrV4 { inner: sock }
  }
}

impl Into<SocketAddr> for MySocketAddrV4 {
  fn into(self) -> SocketAddr {
    SocketAddr::V4(
      SocketAddrV4::new(self.ip().clone(), self.port())
    )
  }
}

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

  fn into_inner(self) -> (*const c::sockaddr, c::socklen_t) {
    (&self as *const _ as *const _, mem::size_of_val(&self) as c::socklen_t)
  }
}

impl From<c::sockaddr_in6> for MySocketAddrV6 {
  fn from(sock: c::sockaddr_in6) -> Self {
    MySocketAddrV6 { inner: sock }
  }
}

impl Into<SocketAddr> for MySocketAddrV6 {
  fn into(self) -> SocketAddr {
    SocketAddr::V6(
      SocketAddrV6::new(self.ip().clone(), self.port(), 0, 0)
    )
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
pub fn ip_to_sockaddr(ip: &IpAddr) ->  (*const c::sockaddr, c::socklen_t) {
  match ip {
    &IpAddr::V4(ipv4) => {
      MySocketAddrV4::new(&ipv4, 0).into_inner()
    },
    &IpAddr::V6(ipv6) => {
      MySocketAddrV6::new(&ipv6, 0, 0, 0).into_inner()
    },
  }
}
