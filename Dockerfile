# Use official Rust image as the base image
FROM rust:1.67-slim as builder

# Set the working directory inside the container
WORKDIR /usr/src/penspecter-server

# Copy Cargo.toml and Cargo.lock first to leverage caching
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies
RUN cargo fetch

# Now copy the rest of the source code and build the final binary
COPY . .

# Build the app
RUN cargo build --release

# Use a smaller runtime base image
FROM debian:bullseye-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/penspecter-server/target/release/penspecter-server /usr/local/bin/penspecter-server

# Set the default command to run the app
CMD ["penspecter-server"]
