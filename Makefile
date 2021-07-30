
SHELL		= bash

NAME		= devhub

DNAREPO			= bundled/dnarepo/dnarepo.dna
DNAREPO_WASM		= target/wasm32-unknown-unknown/release/storage.wasm
HAPPDNA			= bundled/happs/happs.dna
HAPPDNA_WASM		= target/wasm32-unknown-unknown/release/store.wasm
ASSETSDNA		= bundled/web_assets/files.dna
ASSETSDNA_WASM		= target/wasm32-unknown-unknown/release/files.wasm

MERE_MEMORY_BASE	= zomes/mere_memory
MERE_MEMORY_WASM	= $(MERE_MEMORY_BASE)/target/wasm32-unknown-unknown/release/mere_memory.wasm

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
$(DNAREPO):			$(DNAREPO_WASM) $(MERE_MEMORY_WASM)
	@echo "Packaging DNAREPO: $@"
	@hc dna pack $(dir $@)
	@ls -l $(dir $@)

$(DNAREPO_WASM):		Makefile
	@echo "Building  DNAREPO WASM: $@"; \
	cd dnas/dnarepo/; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build \
	    --release --target wasm32-unknown-unknown \
	    --package storage

happdna:			$(HAPPDNA)
$(HAPPDNA):			$(HAPPDNA_WASM)
	@echo "Packaging HAPPDNA: $@"
	@hc dna pack $(dir $@)
	@ls -l $(dir $@)

$(HAPPDNA_WASM):		Makefile
	@echo "Building  HAPPDNA WASM: $@"; \
	cd dnas/happs/; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build \
	    --release --target wasm32-unknown-unknown \
	    --package store

webassetdna:			$(ASSETSDNA)
$(ASSETSDNA):			$(ASSETSDNA_WASM)
	@echo "Packaging ASSETSDNA: $@"
	@hc dna pack $(dir $@)
	@ls -l $(dir $@)

$(ASSETSDNA_WASM):		Makefile
	@echo "Building  ASSETSDNA WASM: $@"; \
	cd dnas/web_assets/; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build \
	    --release --target wasm32-unknown-unknown \
	    --package files


#
# Testing
#
TEST_DNA_MERE_MEMORY	= tests/dnas/memory/memory.dna

test-all:			test-crates test-zomes test-dnas test-multi
test-all-debug:			test-crates test-zomes-debug test-dnas-debug test-multi-debug
test:				test-unit
test-unit:
	cd dnas/dnarepo/; \
	RUST_BACKTRACE=1 cargo test \
	    -- --nocapture
unit-%:
	RUST_BACKTRACE=1 cargo test $* \
	    -- --nocapture
tests/test.dna:
	cp $(DNAREPO) $@
tests/test.gz:
	gzip -kc bundled/dnarepo/dnarepo.dna > $@

# DNAs
test-dnas:			test-dnarepo		test-happs		test-webassets
test-dnas-debug:		test-dnarepo-debug	test-happs-debug	test-webassets-debug

test-dnarepo:			dnarepo
	cd tests; RUST_LOG=none npx mocha integration/test_dnarepo.js
test-dnarepo-debug:		dnarepo
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_dnarepo.js

test-happs:			happdna
	cd tests; RUST_LOG=none npx mocha integration/test_happs.js
test-happs-debug:		happdna
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_happs.js

test-webassets:			webassetdna
	cd tests; RUST_LOG=none npx mocha integration/test_webassets.js
test-webassets-debug:		webassetdna
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_webassets.js

test-multi:			dnarepo happdna webassetdna
	cd tests; RUST_LOG=none npx mocha integration/test_multiple.js
test-multi-debug:		dnarepo happdna webassetdna
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_multiple.js

# Zomes
test-zomes:			test-zome-mere-memory
test-zomes-debug:		test-zome-mere-memory-debug
test-zome-mere-memory:		test_dna_mere_memory
	cd tests; RUST_LOG=none npx mocha integration/test_zome_mere_memory.js
test-zome-mere-memory-debug:	test_dna_mere_memory
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_zome_mere_memory.js

test-crates:
	cd essence_payloads; cargo test
	cd hc_entities; cargo test
	cd dna_utils; cargo test
	cd devhub_types; cargo test
test_dna_mere_memory:		$(TEST_DNA_MERE_MEMORY)
$(TEST_DNA_MERE_MEMORY):	$(MERE_MEMORY_WASM)
	@echo "Packaging test DNA for 'mere_memory' zome: $@"
	@hc dna pack $(dir $@)
	@ls -l $(dir $@)
$(MERE_MEMORY_WASM):		Makefile $(MERE_MEMORY_BASE)/src/*.rs $(MERE_MEMORY_BASE)/Cargo.toml
	@echo "Building zome: $@"; \
	cd $(MERE_MEMORY_BASE); RUST_BACKTRACE=1 cargo build \
		--release --target wasm32-unknown-unknown


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
