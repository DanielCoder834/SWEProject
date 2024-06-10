FROM rust:1.78

WORKDIR /src

RUN openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj /CN=localhost;

# Dependecies
COPY Cargo.toml ./

# Copy the rest of the application code
COPY ../.. .

# Run Rust Code
RUN cargo build

# Expose the port for the server
EXPOSE 5000

#RUN chmod +x
CMD ["cargo run","./src/backend"]