
SHELL			= bash
NAME			= devhub

# External WASM dependencies
MERE_MEMORY_VERSION	= 0.88.0
MERE_MEMORY_WASM	= zomes/mere_memory.wasm
MERE_MEMORY_API_WASM	= zomes/mere_memory_api.wasm

# External DNA dependencies
PORTAL_DNA		= dnas/portal.dna


# hApp
DEVHUB_HAPP		= devhub.happ

# DNAs
ZOMEHUB_DNA		= dnas/zome_hub.dna
DNAHUB_DNA		= dnas/dna_hub.dna
APPHUB_DNA		= dnas/app_hub.dna

# Integrity Zomes
ZOMEHUB_WASM		= zomes/zome_hub.wasm
DNAHUB_WASM		= zomes/dna_hub.wasm
APPHUB_WASM		= zomes/app_hub.wasm

# Coordinator WASMs
ZOMEHUB_CSR_WASM	= zomes/zome_hub_csr.wasm
DNAHUB_CSR_WASM		= zomes/dna_hub_csr.wasm
APPHUB_CSR_WASM		= zomes/app_hub_csr.wasm

TARGET			= release
TARGET_DIR		= target/wasm32-unknown-unknown/release
SOURCE_FILES		= Makefile zomes/Cargo.* zomes/*/Cargo.toml zomes/%/src/*.rs \
				devhub_sdk/Cargo.toml devhub_sdk/src/*.rs


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
	    .cargo target zomes/target \
	    $(DEVHUB_HAPP) \
	    $(ZOMEHUB_DNA) $(DNAHUB_DNA) $(APPHUB_DNA) \
	    $(ZOMEHUB_WASM) $(ZOMEHUB_CSR_WASM) \
	    $(DNAHUB_WASM) $(DNAHUB_CSR_WASM) \
	    $(APPHUB_WASM) $(APPHUB_CSR_WASM) \
	    $(MERE_MEMORY_WASM) $(MERE_MEMORY_API_WASM)

rebuild:			clean build
build:				$(DEVHUB_HAPP)


$(DEVHUB_HAPP):			$(ZOMEHUB_DNA) $(DNAHUB_DNA) $(APPHUB_DNA) happ/happ.yaml
	hc app pack -o $@ ./happ/

$(ZOMEHUB_DNA):			$(ZOMEHUB_WASM) $(ZOMEHUB_CSR_WASM) $(MERE_MEMORY_WASM) $(MERE_MEMORY_API_WASM)
$(DNAHUB_DNA):			$(DNAHUB_WASM) $(DNAHUB_CSR_WASM)
$(APPHUB_DNA):			$(APPHUB_WASM) $(APPHUB_CSR_WASM)

dnas/%.dna:			dnas/%/dna.yaml
	@echo "Packaging '$*': $@"
	@hc dna pack -o $@ dnas/$*

zomes/%.wasm:			zomes/$(TARGET_DIR)/%.wasm
	@echo -e "\x1b[38;2mCopying WASM ($<) to 'zomes' directory: $@\x1b[0m"; \
	cp $< $@
zomes/$(TARGET_DIR)/%.wasm:	$(SOURCE_FILES)
	rm -f zomes/$*.wasm
	@echo -e "\x1b[37mBuilding zome '$*' -> $@\x1b[0m";
	cd zomes; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build --release \
	    --target wasm32-unknown-unknown \
	    --package $*
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time

use-local-backdrop:
	cd tests; npm uninstall @whi/holochain-backdrop
	cd tests; npm install --save-dev ../../node-holochain-backdrop
use-npm-backdrop:
	cd tests; npm uninstall @whi/holochain-backdrop
	cd tests; npm install --save-dev @whi/holochain-backdrop
use-local-client:
	cd tests; npm uninstall @whi/holochain-client
	cd tests; npm install --save-dev ../../js-holochain-client
use-npm-client:
	cd tests; npm uninstall @whi/holochain-client
	cd tests; npm install --save-dev @whi/holochain-client
use-local-crux:
	cd tests; npm uninstall @whi/crux-payload-parser
	cd tests; npm install --save-dev ../../js-crux-payload-parser
use-npm-crux:
	cd tests; npm uninstall @whi/crux-payload-parser
	cd tests; npm install --save-dev @whi/crux-payload-parser

use-local:		use-local-client use-local-backdrop
use-npm:		  use-npm-client   use-npm-backdrop

copy-portal-from-local:
	cp ../portal-dnas/bundled/portal.dna $(PORTAL_DNA)

$(PORTAL_DNA):
	wget -O $@ "https://github.com/holochain/portal-dna/releases/download/v$(NEW_PORTAL_VERSION)/portal.dna" || rm -f $(PORTAL_DNA)

$(MERE_MEMORY_WASM):
	curl --fail -L "https://github.com/mjbrisebois/hc-zome-mere-memory/releases/download/v$(MERE_MEMORY_VERSION)/mere_memory.wasm" --output $@
$(MERE_MEMORY_API_WASM):
	curl --fail -L "https://github.com/mjbrisebois/hc-zome-mere-memory/releases/download/v$(MERE_MEMORY_VERSION)/mere_memory_api.wasm" --output $@



#
# Testing
#
test:				test-unit-all test-dnas
test-debug:			test-unit-all test-dnas-debug

test-unit-all:			test-unit test-unit-dna_library test-unit-happ_library test-unit-web_assets
test-unit:
	cd devhub_types;	RUST_BACKTRACE=1 cargo test
test-unit-%:
	cd zomes;		RUST_BACKTRACE=1 cargo test $* -- --nocapture

test-integration-debug:
	make test-zome-hub-integration-debug;

test-zome-hub-integration:		$(ZOMEHUB_DNA)
	cd tests; LOG_LEVEL=warn npx mocha ./integration/test_zome_hub.js
test-zome-hub-integration-debug:	$(ZOMEHUB_DNA)
	cd tests; LOG_LEVEL=trace npx mocha ./integration/test_zome_hub.js

tests/test.dna:
	cp $(DNAREPO) $@
tests/test.gz:
	gzip -kc $(DNAREPO) > $@

# DNAs
test-setup:			tests/node_modules



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
