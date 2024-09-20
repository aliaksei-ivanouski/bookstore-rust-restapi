#
# Build
#
FROM rust:1.81.0-slim-bullseye AS build
WORKDIR /rust-rocket-restapi
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

#
# Run
#
FROM debian:bullseye-slim AS final
WORKDIR /rust-rocket-restapi
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=build /rust-rocket-restapi/target/release/rust-rocket-restapi ./rust-rocket-restapi
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

ENTRYPOINT [ "./rust-rocket-restapi" ]
