FROM rust:1.67
COPY . .
WORKDIR /
RUN cargo build --release
EXPOSE 8080
CMD ["./target/release/assure-2035-rust-api"]