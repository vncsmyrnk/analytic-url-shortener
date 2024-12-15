FROM rust:1.83-bookworm AS build
WORKDIR /var/app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build /var/app/target/release/aus /app
CMD ["./app"]
