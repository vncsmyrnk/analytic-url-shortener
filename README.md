![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

# Analytic Url Shortener

Shorten URLs and view metrics. Inspired by [grabify](https://grabify.link).

## Live environment

[Being implemented!](https://github.com/vncsmyrnk/analytic-url-shortener/issues) 🔜

## Run locally

```sh
make dep run-migrations run
```

### With docker

```sh
docker run --rm -it -p 8080:8080 -e DATABASE_URL=postgres://user:host/database ghcr.io/vncsmyrnk/analytic-url-shortener
```

Be sure to run the migrations in order to make the database usable: `$ make run-migrations`
