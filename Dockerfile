FROM alpine

# Add the required environment variables
ENV DOCKER=1

# Copy the binary to the container
COPY target/x86_64-unknown-linux-musl/release/migrator /

# Run the binary
ENTRYPOINT ["/migrator"]