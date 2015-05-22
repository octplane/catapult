FROM schickling/rust

ADD . /source
RUN cargo build --release
