FROM rust:1.31

WORKDIR /src/main
COPY . .

RUN cargo install --path .

CMD ["main"]
