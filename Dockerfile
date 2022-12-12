FROM ubuntu:latest as build

RUN apt-get update && apt-get install build-essential pkg-config libssl-dev curl -y
# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# create a new empty shell project
RUN $HOME/.cargo/bin/cargo new --bin pgdelaytest
WORKDIR /pgdelaytest

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# copy your source tree
COPY ./src ./src

# build for release
RUN $HOME/.cargo/bin/cargo build --release
RUN $HOME/.cargo/bin/cargo build

# our final base
FROM ubuntu:latest

RUN apt-get update && apt-get dist-upgrade -y && apt-get install ca-certificates libssl-dev tini tzdata -y

# copy the build artifact from the build stage
COPY --from=build /pgdelaytest/target/release/pgdelaytest /usr/local/bin/.
COPY --from=build /pgdelaytest/target/release/pgdelaytest /usr/local/bin/pgdelaytest-debug

ENV RUST_LOG=info
ENTRYPOINT ["/usr/bin/tini", "--", "/usr/local/bin/pgdelaytest"]
