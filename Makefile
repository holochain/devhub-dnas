
SHELL		= bash

NAME		= devhub

DNAREPO		= bundled/dnas/dnas.dna
DNAREPO_WASM	= target/wasm32-unknown-unknown/release/storage.wasm
HAPPDNA		= bundled/happs/happs.dna
HAPPDNA_WASM	= target/wasm32-unknown-unknown/release/store.wasm


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
	    $(DNAREPO)
	    $(HAPPDNA)
rebuild:			clean build
build:				dnarepo happdna

dnarepo:			$(DNAREPO)
$(DNAREPO):			$(DNAREPO_WASM)
	@echo "Packaging DNAREPO: $@"
	@hc dna pack $(dir $@)
	@ls -l $(dir $@)

$(DNAREPO_WASM):	Makefile
	@echo "Building  DNAREPO WASM: $@"; \
	cd recipes/dnas/; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build \
	    --release --target wasm32-unknown-unknown \
	    --package storage

happdna:			$(HAPPDNA)
$(HAPPDNA):			$(HAPPDNA_WASM)
	@echo "Packaging HAPPDNA: $@"
	@hc dna pack $(dir $@)
	@ls -l $(dir $@)

$(HAPPDNA_WASM):	Makefile
	@echo "Building  HAPPDNA WASM: $@"; \
	cd recipes/happs/; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build \
	    --release --target wasm32-unknown-unknown \
	    --package store


#
# Testing
#
test-all:			test
test:				test-unit test-e2e
test-unit:
	cd recipes/dnas/; \
	RUST_BACKTRACE=1 cargo test \
	    -- --nocapture
unit-%:
	RUST_BACKTRACE=1 cargo test $* \
	    -- --nocapture
tests/test.dna:
	cp $(DNAREPO) $@
test-dnas-debug:		tests/node_modules $(DNAREPO) tests/test.dna
	cd tests; \
	RUST_LOG=[debug]=debug TRYORAMA_LOG_LEVEL=info RUST_BACKTRACE=full TRYORAMA_HOLOCHAIN_PATH="holochain" node src/test_dnas.js
test-happs-debug:		tests/node_modules $(HAPPDNA)
	cd tests; \
	RUST_LOG=[debug]=debug TRYORAMA_LOG_LEVEL=info RUST_BACKTRACE=full TRYORAMA_HOLOCHAIN_PATH="holochain" node src/test_happs.js
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
