# Use official Rust image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/penspecter-server

# Copy the Cargo.toml and Cargo.lock files and build dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs so Cargo can fetch dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Fetch dependencies
RUN cargo build --release

# Now copy the source code and build the final binary
COPY . .

# Build the app
RUN cargo build --release

# Use a lighter base image to run the app (alpine)
FROM alpine:latest

# Install necessary dependencies (libgcc, libstdc++, libc6-compat)
RUN apk add --no-cache libc6-compat libgcc libstdc++

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/penspecter-server/target/release/penspecter-server /usr/local/bin/penspecter-server

# Set the default command to run the app
CMD ["penspecter-server"]
