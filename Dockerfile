FROM rust:1.82.0 AS builder
WORKDIR /root

# Create a new empty project
COPY Cargo.toml Cargo.lock ./

# Build dependencies only
# RUN apt-get update -y && \
#   apt-get install -y pkg-config libssl-dev libudev-dev perl-base libfindbin-libs-perl && \
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Build the actual project
COPY . .
RUN touch src/main.rs && \
  cargo build --release

FROM ubuntu
WORKDIR /root
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /root/target/release/weather-alert .
#ENV LOG=info,warn,error
ENTRYPOINT ["./weather-alert"]
