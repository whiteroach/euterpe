FROM rust:1.66.1 as build
COPY . /euterpe
WORKDIR /euterpe

RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/root/target \
    cargo build --release

FROM debian:buster-slim as runtime
COPY --from=build /euterpe/target/release/euterpe .
CMD ["./euterpe"]