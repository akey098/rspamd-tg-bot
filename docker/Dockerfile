# Stage 1: Build your application
FROM ubuntu:latest as builder
LABEL authors="fumik"
WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y build-essential curl pkg-config libssl-dev

# Install Rust (non-interactive install)
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy your source code and custom rspamd config
COPY . .

# Copy custom rspamd config (if needed in the build stage)
COPY rspamd-config/local.d/ /etc/rspamd/local.d/

# Run rspamd config test if required (ensure rspamadm is installed or available)
RUN if command -v rspamadm; then rspamadm configtest; fi

# Build your application in release mode
RUN cargo build --release

# Stage 2: Create a lean runtime image
FROM ubuntu:latest
LABEL authors="fumik"
WORKDIR /app

# Copy only the built binary from the builder stage
COPY --from=builder /app/target/release/your_binary /usr/local/bin/your_binary

# (Optional) Copy additional runtime configuration if needed
COPY rspamd-config/local.d/ /etc/rspamd/local.d/

# Set the entrypoint to your binary
ENTRYPOINT ["your_binary"]
