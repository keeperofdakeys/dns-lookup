# dns_lookup
A small libc::getaddrinfo wrapper for Rust to perform dns lookups.

[Documentation](https://keeperofdakeys.github.io/dns-lookup/dns_lookup)

## Usage
```rust
use dns_lookup::lookup;

lookup::lookup_host("hostname");
```
