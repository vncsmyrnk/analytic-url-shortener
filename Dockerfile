FROM rust:1.83-bookworm AS build
WORKDIR /var/app
COPY . .
RUN <<EOF
cargo build --release
ls target/release/**
EOF

FROM gcr.io/distroless/static-debian12
COPY --from=build /var/app/target/release/aus /app
ENTRYPOINT ["/app"]
