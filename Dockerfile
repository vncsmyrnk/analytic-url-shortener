FROM rust:1.83-bookworm AS build
WORKDIR /var/app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/static-debian12
COPY --from=build /var/app/target/release/aus /
ENTRYPOINT ["/aus"]
