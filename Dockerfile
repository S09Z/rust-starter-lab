# Dockerfile

FROM rust:latest

# Create a new directory for the app
WORKDIR /usr/src/my_chat_app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a new project
RUN cargo build --release
RUN mkdir src

# Copy the source code
COPY src ./src

# Build the app
CMD ["cargo", "run"]
