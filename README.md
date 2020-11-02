# r2d2-ldap
[![Build](https://github.com/c0dearm/r2d2-ldap/workflows/Rust/badge.svg?branch=main)](https://github.com/c0dearm/r2d2-ldap/actions)
[![Latest Version](https://img.shields.io/crates/v/r2d2-ldap.svg)](https://crates.io/crates/r2d2-ldap)
[![Docs](https://docs.rs/r2d2-ldap/badge.svg)](https://docs.rs/r2d2-ldap)

LDAP support for the r2d2 connection pool

## Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
r2d2-ldap = "0.1.0"
```

## Basic Usage

```rust
use std::thread;
use r2d2_ldap::LDAPConnectionManager;

fn main() {
    let pool = r2d2::Pool::new(LDAPConnectionManager("ldap://example.org")).unwrap();
    let mut ldap = pool.get().unwrap();
    ldap.simple_bind("uid=john,cn=users,dc=example,dc=org", "password").unwrap();
}
```

## License
 * MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
 * This software includes the work that is distributed in the Apache License 2.0
