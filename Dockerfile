FROM rust:1.82-alpine AS builder
WORKDIR /usr/src
COPY . .
RUN cargo install --path services/entrypoint
# CMD ["bash"]

FROM scratch
COPY --from=builder /usr/local/cargo/bin/inglix /usr/local/bin/inglix
CMD ["inglix"]
