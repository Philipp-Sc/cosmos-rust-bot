FROM rust:latest
 
WORKDIR /usr/workspace

COPY src src/
COPY Cargo.toml .
RUN cargo build

RUN mkdir ./static 

EXPOSE 8081
CMD ["cargo run"]

# sudo docker build -t server .
# sudo docker run --rm -p 8081:8081 -v "$(pwd)../../../../tmp":/usr/workspace/tmp  cosmos_rust_server cargo run
