FROM messense/rust-musl-cross:x86_64-musl as builder
WORKDIR /assist-lamfo
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM debian
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates
COPY --from=builder /assist-lamfo/robert /robert
COPY --from=builder /assist-lamfo/sql /sql
COPY --from=builder /assist-lamfo/target/x86_64-unknown-linux-musl/release/assist-lamfo /assist-lamfo
ENTRYPOINT ["/assist-lamfo"]
EXPOSE 8000