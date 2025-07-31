FROM messense/rust-musl-cross:x86_64-musl as builder
WORKDIR /assist-lamfo
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM debian
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates

COPY --from=builder /assist-lamfo/lamfo_gpt /lamfo_gpt
COPY --from=builder /assist-lamfo/sql /sql
COPY --from=builder /assist-lamfo/target/x86_64-unknown-linux-musl/release/assist-lamfo /assist-lamfo

# Copia o script para a imagem final e dá permissão
COPY shell_script/my_entrypoint.sh /my_entrypoint.sh
RUN chmod +x /my_entrypoint.sh

ENTRYPOINT ["/my_entrypoint.sh"]
EXPOSE 8000
