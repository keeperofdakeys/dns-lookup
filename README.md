# dns_lookup
A small wrapper for libc to perform simple DNS lookups.

You can use the `lookup_host` function to get a list of IP Addresses for a
given hostname, and the `lookup_name` function to get the reverse dns entry for
the given IP Address.


[Documentation](https://keeperofdakeys.github.io/dns-lookup/dns_lookup)

## Usage
```rust
use dns_lookup::{lookup_host, lookup_addr};

{
  let hostname = "localhost";
  let ips: Vec<std::net::IpAddr> =
    lookup_host(hostname).unwrap().collect::<std::io::Result<_>>().unwrap();
  assert!(ips.contains(&"127.0.0.1".parse().unwrap()));
}

{
  use dns_lookup::{lookup_host, lookup_addr};

  let ip: std::net::IpAddr = "127.0.0.1".parse().unwrap();
  let hostname = lookup_addr(&ip).unwrap();
  assert_eq!(hostname, "localhost");
}
```
