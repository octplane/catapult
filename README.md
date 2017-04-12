# Catapult

Catapult is a small replacement for logstash. It has been designed to read logs in various
places and send them to other places.

Catapult is used in production to fetch Docker logs from container and send them
to a central location.

# Usage

Catapult runs with a configuration that describes how to use it.
This configuration is passed to the binary via the `-c` option.

Please refer to the [API documentation](http://people.zoy.org/~oct/public/doc/catapult/)

# Installing

```
# on OSX only
export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include 
cargo build --release
```

or

```
make
```

If you want to build the linux binary inside a container

# Contributors

- Łukasz Niemier / hauleth : Backport to support beta rust

# Licence

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
