FROM rust:latest
COPY . .
WORKDIR /
RUN cargo build --release
EXPOSE 8080
CMD ["./target/release/assure-2035-rust-api"]