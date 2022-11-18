FROM rustlang/rust:nightly as build

# create a new empty shell project
RUN USER=root cargo new --bin pgdelaytest
WORKDIR /pgdelaytest

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# copy your source tree
COPY ./src ./src

# build for release
RUN cargo build --release

# our final base
FROM ubuntu:latest

RUN apt-get update
RUN apt-get dist-upgrade -y
RUN apt-get install ca-certificates libssl-dev tini tzdata -y

# copy the build artifact from the build stage
COPY --from=build /pgdelaytest/target/release/pgdelaytest /usr/local/bin/.

ENV RUST_LOG=info
ENTRYPOINT ["/usr/bin/tini", "--", "/usr/local/bin/pgdelaytest"]
