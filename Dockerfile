FROM lukemathwalker/cargo-chef:latest-rust-1.59.0-slim-buster as chef
WORKDIR /app
RUN apt update && apt install lld clang git -y
FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
# Build our project
RUN cargo build --release --bin chains_bot
#RUN sea-orm-cli migrate up

# turtle: we can't use a fresh runtime just for the executable because we need to run migrations with the project tree
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
   && apt-get install -y --no-install-recommends openssl git ca-certificates \
   # Clean up
   && apt-get autoremove -y \
   && apt-get clean -y \
   && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/chains_bot chains_bot
RUN git clone https://github.com/vishnubob/wait-for-it.git
EXPOSE 8080

#ENTRYPOINT ["tail", "-f", "/dev/null"]
CMD ["./chains_bot"]