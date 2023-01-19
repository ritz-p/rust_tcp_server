FROM rust:latest

RUN apt update
RUN apt install vim -y
WORKDIR /workspace
ENV RUST_BACKTRACE=1
RUN rustup install nightly
RUN rustup component add rustfmt
RUN cargo install sccache
RUN RUSTC_WRAPPER=$(which sccache)
ENV RUSTC_WRAPPER=${RUSTC_WRAPPER}
