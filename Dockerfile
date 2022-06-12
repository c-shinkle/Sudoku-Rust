# syntax=docker/dockerfile:1

FROM rust:1.61 as builder
WORKDIR /usr/src/sudoku_rust
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/sudoku_rust /usr/local/bin/sudoku_rust
ADD ./res/bench.txt res/bench.txt
CMD ["sudoku_rust"]