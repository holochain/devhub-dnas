
SHELL		= bash

NAME		= devhub
DNA		= bundled/dnas/dnas.dna
DNA_WASM	= target/wasm32-unknown-unknown/release/storage.wasm


#
# Project
#
tests/package-lock.json:	tests/package.json
	touch $@
tests/node_modules:		tests/package-lock.json
	cd tests; \
	npm install
	touch $@
clean:
	rm -rf \
	    tests/node_modules \
	    .cargo \
	    target \
	    $(DNA)
rebuild:	clean build
build:		dna

dna:		$(DNA)
$(DNA):		$(DNA_WASM)
	@echo "Packaging DNA: $@"
	@hc dna pack $(dir $@)
	@ls -l $(dir $@)

$(DNA_WASM):	Makefile
	@echo "Building  DNA WASM: $@"; \
	cd recipes/dnas/; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build \
	    --release --target wasm32-unknown-unknown \
	    --package storage


#
# Testing
#
test-all:	test
test:		test-unit test-e2e
test-unit:
	cd recipes/dnas/; \
	RUST_BACKTRACE=1 cargo test \
	    -- --nocapture
unit-%:
	RUST_BACKTRACE=1 cargo test $* \
	    -- --nocapture
tests/test.dna:
	cp $(DNA) $@
test-dnas-debug:	tests/node_modules $(DNA) tests/test.dna
	cd tests; \
	RUST_LOG=[debug]=debug TRYORAMA_LOG_LEVEL=info RUST_BACKTRACE=full TRYORAMA_HOLOCHAIN_PATH="holochain" node src/test_dnas.js
test-crates:
	cd essence_payloads; cargo test
	cd devhub_types; cargo test
	cd dna_utils; cargo test


#
# Repository
#
clean-remove-chaff:
	@find . -name '*~' -exec rm {} \;
clean-files:		clean-remove-chaff
	git clean -nd
clean-files-force:	clean-remove-chaff
	git clean -fd
clean-files-all:	clean-remove-chaff
	git clean -ndx
clean-files-all-force:	clean-remove-chaff
	git clean -fdx
