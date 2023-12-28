FROM rust:1.74.1-slim-buster as build

RUN USER=root cargo new --bin vigilant-archive
WORKDIR /vigilant-archive

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./static ./static
COPY ./templates ./templates

RUN rm ./target/release/deps/vigilant_archive*
RUN cargo build --release

FROM gcr.io/distroless/cc

EXPOSE 3000

COPY --from=build /vigilant-archive/target/release/vigilant_archive .

CMD ["./vigilant_archive"]
