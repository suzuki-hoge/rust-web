#
# docker
#

up:
	@docker compose up --build --detach

down:
	@docker compose down

db:
	@docker compose exec db mysql -h localhost -u app -psecret sales

#
# cargo
#

run:
	@cargo run -- --thread 2 --connection 2

build:
	@cargo build --release

test:
	@cargo test

fix:
	@cargo +nightly fmt
	@cargo fix --allow-dirty --allow-staged
	@cargo clippy --fix --allow-dirty --allow-staged
	@cargo test
