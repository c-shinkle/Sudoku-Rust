# syntax=docker/dockerfile:1

FROM rust:1.61 as builder
WORKDIR /Sudoku-Rust
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
WORKDIR /Sudoku-Rust
COPY --from=builder /usr/local/cargo/bin/sudoku_rust /usr/local/bin/sudoku_rust
COPY ./res/bench.txt res/bench.txt
CMD ["sudoku_rust"]