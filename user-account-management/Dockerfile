FROM rust:1.66.0

WORKDIR /usr/src/user-account-management
COPY . .

RUN apt update && apt upgrade -y
RUN apt install -y protobuf-compiler libprotobuf-dev
RUN cargo install cargo-watch

EXPOSE 50051

CMD ["cargo", "watch", "-x", "'run'"]