#
# cargo
#

run:
	@cargo run

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
	@curl -i localhost:9090/item/all

201:
	@curl -i -X POST localhost:9090/item/create -d 'code=1234'

400:
	@curl -i -X POST localhost:9090/item/create

404:
	@curl -i localhost:9090/item/foo

500:
	@curl -i localhost:9090/error
