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
cargo build --release
```

or

```
make
```

If you want to build the linux binary inside a container
