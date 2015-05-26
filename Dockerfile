FROM schickling/rust

ADD . /source
RUN cargo build --release
RUN mkdir -p /usr/share/catapult/bin
RUN cp ./target/release/catapult /usr/share/catapult/bin/
VOLUME ["/usr/share/catapult"]
