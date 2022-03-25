# heavily inspired by https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

FROM rust:1.59.0-slim-buster as build

# create a new empty shell project
RUN USER=root cargo new --bin chains_bot
WORKDIR /chains_bot

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./db/Cargo.lock ./db/Cargo.lock
COPY ./db/Cargo.toml ./db/Cargo.toml
COPY ./migration/Cargo.toml ./migration/Cargo.toml

# build fake dependency libs
RUN touch src/main.rs && mkdir -p db/src && touch db/src/lib.rs && mkdir -p migration/src && touch migration/src/lib.rs

# this build step will cache your dependencies
#RUN rm src/*.rs & rm migration/src/*.rs & db/src/*.rs & cargo build --release
RUN cargo build --release

# copy your source tree
COPY ./src ./src
COPY ./migration ./migration
COPY ./db ./db

# build for release
RUN rm ./target/release/chains_bot*
RUN cargo build --release

# our final base
FROM rust:1.59.0-slim-buster

# copy the build artifact from the build stage
COPY --from=build /chains_bot/target/release/chains_bot .

# startup app
CMD ["./chains_bot"]
