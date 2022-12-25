FROM rust:1.66.0

WORKDIR /usr/src/user-co2-management
COPY . .

RUN apt update && apt upgrade -y
RUN apt install -y protobuf-compiler libprotobuf-dev
RUN cargo install cargo-watch

EXPOSE 50052

CMD ["cargo", "watch", "-x", "'run'"]