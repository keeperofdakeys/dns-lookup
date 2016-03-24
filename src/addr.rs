use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, Ipv4Addr, Ipv6Addr};
use libc as c;

pub struct MySocketAddrV4 {
  inner: c::sockaddr_in
}

impl MySocketAddrV4 {
  fn ip(&self) -> &Ipv4Addr {
      unsafe {
          &*(&self.inner.sin_addr as *const c::in_addr as *const Ipv4Addr)
      }
  }

  fn port (&self) -> u16 {
    self.inner.sin_port
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

pub struct MySocketAddrV6 {
  inner: c::sockaddr_in6
}

impl MySocketAddrV6 {
  fn ip(&self) -> &Ipv6Addr {
      unsafe {
          &*(&self.inner.sin6_addr as *const c::in6_addr as *const Ipv6Addr)
      }
  }

  fn port (&self) -> u16 {
    self.inner.sin6_port
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
