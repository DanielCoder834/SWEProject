FROM rust:1.78

WORKDIR /app

RUN openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj /CN=localhost;

# Copy the rest of the application code
COPY . .

# Run Rust Code
# TODO: Add --release later to makefile
RUN cargo build --release

# Expose the port for the server
EXPOSE 9443

#CMD ["./target/release/bins"]
CMD ["cargo", "run"]