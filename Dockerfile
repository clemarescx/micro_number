FROM rust:latest
LABEL Name=bleep_server Version=0.0.1

WORKDIR /usr/src/bleep_server
COPY . .

RUN cargo install --path .

CMD ["bleep_server"]
