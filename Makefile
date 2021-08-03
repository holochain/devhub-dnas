
SHELL		= bash

NAME		= devhub

DNAREPO			= bundled/dnarepo/dnarepo.dna
HAPPDNA			= bundled/happs/happs.dna
ASSETSDNA		= bundled/web_assets/web_assets.dna

DNA_LIBRARY_WASM	= target/wasm32-unknown-unknown/release/dna_library.wasm
HAPP_LIBRARY_WASM	= target/wasm32-unknown-unknown/release/happ_library.wasm
WEB_ASSETS_WASM		= target/wasm32-unknown-unknown/release/web_assets.wasm

MERE_MEMORY_BASE	= zomes/mere_memory
MERE_MEMORY_WASM	= zomes/target/wasm32-unknown-unknown/release/mere_memory.wasm

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
$(DNAREPO):			$(DNA_LIBRARY_WASM) $(MERE_MEMORY_WASM)
	@echo "Packaging DNAREPO: $@"
	@hc dna pack $(dir $@)
	@ls -l $(dir $@)

$(DNA_LIBRARY_WASM):		Makefile zomes/dna_library/src/*.rs zomes/dna_library/Cargo.toml
	@echo "Building  'dna_library' WASM: $@"; \
	cd zomes; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build \
	    --release --target wasm32-unknown-unknown \
	    --package dna_library

happdna:			$(HAPPDNA)
$(HAPPDNA):			$(HAPP_LIBRARY_WASM)
	@echo "Packaging HAPPDNA: $@"
	@hc dna pack $(dir $@)
	@ls -l $(dir $@)

$(HAPP_LIBRARY_WASM):		Makefile zomes/happ_library/src/*.rs zomes/happ_library/Cargo.toml
	@echo "Building  'happ_library' WASM: $@"; \
	cd zomes; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build \
	    --release --target wasm32-unknown-unknown \
	    --package happ_library

webassetdna:			$(ASSETSDNA)
$(ASSETSDNA):			$(WEB_ASSETS_WASM)
	@echo "Packaging ASSETSDNA: $@"
	@hc dna pack $(dir $@)
	@ls -l $(dir $@)

$(WEB_ASSETS_WASM):		Makefile zomes/web_assets/src/*.rs zomes/web_assets/Cargo.toml
	@echo "Building  'web_assets' WASM: $@"; \
	cd zomes; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build \
	    --release --target wasm32-unknown-unknown \
	    --package web_assets

mere-memory-zome:		$(MERE_MEMORY_WASM)
	cd zomes; cargo publish --dry-run --manifest-path mere_memory/Cargo.toml
$(MERE_MEMORY_WASM):		Makefile $(MERE_MEMORY_BASE)/src/*.rs $(MERE_MEMORY_BASE)/Cargo.toml
	@echo "Building zome: $@"; \
	cd zomes; RUST_BACKTRACE=1 cargo build --package hc_zome_mere_memory \
		--release --target wasm32-unknown-unknown



#
# Testing
#
TEST_DNA_MERE_MEMORY	= tests/dnas/memory/memory.dna

test-all:			test-crates test-zomes test-dnas test-multi
test-all-debug:			test-crates test-zomes-debug test-dnas-debug test-multi-debug
test:				test-unit
test-unit:
	cd zomes/dna_library/; \
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


#
# Documentation
#
build-docs:			build-mere-memory-docs
build-mere-memory-docs:
	cd zomes; cargo doc -p hc_zome_mere_memory


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
