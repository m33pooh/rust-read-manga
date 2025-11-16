# Makefile with common tasks (use in WSL or Git Bash; on Windows PowerShell use equivalent commands shown in README)

.PHONY: build up down logs run

build:
	cargo build --manifest-path=service/Cargo.toml

up:
	docker compose up -d

down:
	docker compose down

logs:
	docker compose logs -f

run:
	cargo run --manifest-path=service/Cargo.toml
