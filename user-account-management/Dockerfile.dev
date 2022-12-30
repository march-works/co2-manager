FROM rust:1.66.0

WORKDIR /usr/src/user-account-management

RUN apt update && apt upgrade -y
RUN apt install -y protobuf-compiler libprotobuf-dev
RUN cargo install cargo-watch

COPY . .
CMD ["cargo", "watch", "-x", "'run'"]