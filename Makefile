.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream:
	substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml map_collection_data_entity_changes -s 14961455 -t +15

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"
