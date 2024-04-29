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

test:
	@cargo test

fix:
	@cargo +nightly fmt
	@cargo fix --allow-dirty --allow-staged
	@cargo clippy --fix --allow-dirty --allow-staged
	@cargo test

#
# curl
#

200:
	@curl -sS localhost:9090/item/all | jq . -c

201:
	@curl -sS -X POST localhost:9090/item/create -d 'code=1234' | jq . -c

400:
	@curl -sS -X POST localhost:9090/item/create | jq . -c

404:
	@curl -sS localhost:9090/item/foo | jq . -c

500:
	@curl -sS localhost:9090/error | jq . -c

sleep:
	@curl -sS localhost:9090/sleep | jq . -c
