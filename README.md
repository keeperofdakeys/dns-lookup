# dns_lookup
A small wrapper for libc to perform simple DNS lookups.

You can use the `lookup_host` function to get a list of IP Addresses for a
given hostname, and the `lookup_name` function to get the reverse dns entry for
the given IP Address.


[Documentation](https://keeperofdakeys.github.io/dns-lookup/dns_lookup)

## Usage

### Simple API

```rust
use dns_lookup::{lookup_host, lookup_addr};

{
  let hostname = "localhost";
  let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();
  assert!(ips.contains(&"127.0.0.1".parse().unwrap()));
}

{
  use dns_lookup::{lookup_host, lookup_addr};

  let ip: std::net::IpAddr = "127.0.0.1".parse().unwrap();
  let hostname = lookup_addr(&ip).unwrap();
  assert_eq!(hostname, "localhost");
}
```

### libc API
```rust
use dns_lookup::{getaddrinfo, AddrInfoHints};

{
  use dns_lookup::{getaddrinfo, AddrInfoHints};

  let hostname = "localhost";
  let service = "ssh";
  let hints = AddrInfoHints {
    socktype: dns_lookup::SockType::Stream,
    .. AddrInfoHints::default()
  };
  let sockets =
    getaddrinfo(Some(hostname), Some(service), Some(hints))
      .unwrap().collect::<std::io::Result<Vec<_>>>().unwrap();
  println!("{:?}", sockets);
  for socket in sockets {
    // Try connecting to socket
    println!("{:?}", socket);
  }

  {
    use dns_lookup::getnameinfo;
    use std::net::{IpAddr, SocketAddr};

    let ip: IpAddr = "127.0.0.1".parse().unwrap();
    let port = 22;
    let socket: SocketAddr = (ip, port).into();

    let (name, service) = match getnameinfo(&socket, 0) {
      Ok((n, s)) => (n, s),
      Err(e) => panic!("Failed to lookup socket {:?}", e),
    };

    println!("{:?} {:?}", name, service);
    let _ = (name, service);
  }
}
