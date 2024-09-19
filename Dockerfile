#
# Build
#
FROM rust:1.81.0-slim-bullseye AS build
WORKDIR /bookstore
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

#
# Run
#
FROM debian:bullseye-slim
WORKDIR /bookstore
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=build /bookstore/target/release/bookstore ./bookstore
#EXPOSE 80

#
# And away we go...
#
ENTRYPOINT [ "./bookstore" ]
