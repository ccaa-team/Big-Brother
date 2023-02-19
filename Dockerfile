FROM rust:1.67

WORKDIR /usr/src/autovirt
COPY . .

RUN cargo install --path .

CMD ["autovirt"]
