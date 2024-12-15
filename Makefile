run:
	cargo watch -x run

build:
	cargo build --release

dep:
	@docker compose up -d

down:
	@docker compose stop

run-migrations:
	diesel migration run
