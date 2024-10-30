set dotenv-load

check:
	cargo stylus check

gas:
	cargo stylus deploy --private-key-path=$PRIV_KEY_PATH --estimate-gas-only

deploy:
	cargo stylus deploy --private-key-path=$PRIV_KEY_PATH

test-onchain:
	cargo run --example journal_test
