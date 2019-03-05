FROM rust:1.31

WORKDIR /src/elma
COPY . .

RUN cargo install --path .

CMD ["elma"]
