FROM rust:latest

RUN apt update
RUN apt install vim -y
WORKDIR /workspace
ENV RUST_BACKTRACE=1