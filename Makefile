run:
	cargo watch -x run

dep:
	@docker compose up -d

down:
	@docker compose stop

run-migrations:
	diesel migration run
