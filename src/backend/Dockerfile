FROM rust:1.78

WORKDIR /src

# Dependecies
COPY Cargo.toml ./

RUN cargo install --path .

# Copy the rest of the application code
COPY ../.. .

# Run Rust Code
RUN cargo run

# Expose the port for the server
EXPOSE 5000

CMD ["cargo run","./src/backend"]