
SHELL			= bash
NAME			= devhub

# External WASM dependencies
MERE_MEMORY_VERSION	= 0.90.0
MERE_MEMORY_WASM	= zomes/mere_memory.wasm
MERE_MEMORY_API_WASM	= zomes/mere_memory_api.wasm

# External DNA dependencies
PORTAL_VERSION		= 0.8.0
PORTAL_DNA		= dnas/portal.dna


# hApp
DEVHUB_HAPP		= happ/$(NAME).happ

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
COMMON_SOURCE_FILES	= Makefile zomes/Cargo.toml
INT_SOURCE_FILES	= $(COMMON_SOURCE_FILES) \
				dnas/%/entry_types/Cargo.toml dnas/%/entry_types/src/*.rs \
				zomes/%/Cargo.toml zomes/%/src/*.rs
CSR_SOURCE_FILES	= $(COMMON_SOURCE_FILES) $(INT_SOURCE_FILES) \
				zomes/%_csr/Cargo.toml zomes/%_csr/src/*.rs \
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

zomes/$(TARGET_DIR)/%.wasm:	$(INT_SOURCE_FILES)
	rm -f zomes/$*.wasm
	@echo -e "\x1b[37mBuilding zome '$*' -> $@\x1b[0m";
	cd zomes; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build --release \
	    --target wasm32-unknown-unknown \
	    --package $*
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time
zomes/$(TARGET_DIR)/%_csr.wasm:	$(CSR_SOURCE_FILES)
	rm -f zomes/$*_csr.wasm
	@echo -e "\x1b[37mBuilding zome '$*_csr' -> $@\x1b[0m";
	cd zomes; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build --release \
	    --target wasm32-unknown-unknown \
	    --package $*_csr
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time

copy-portal-from-local:
	cp ../portal-dnas/bundled/portal.dna $(PORTAL_DNA)

$(PORTAL_DNA):
	wget -O $@ "https://github.com/holochain/portal-dna/releases/download/v$(PORTAL_VERSION)/portal.dna" || rm -f $(PORTAL_DNA)

$(MERE_MEMORY_WASM):
	curl --fail -L "https://github.com/mjbrisebois/hc-zome-mere-memory/releases/download/v$(MERE_MEMORY_VERSION)/mere_memory.wasm" --output $@
$(MERE_MEMORY_API_WASM):
	curl --fail -L "https://github.com/mjbrisebois/hc-zome-mere-memory/releases/download/v$(MERE_MEMORY_VERSION)/mere_memory_api.wasm" --output $@

reset-mere-memory:
	rm zomes/mere_memory*.wasm
	make $(MERE_MEMORY_WASM) $(MERE_MEMORY_API_WASM)

PRE_MM_VERSION = mere_memory_types = "0.89.1"
NEW_MM_VERSION = mere_memory_types = "0.90.0"

PRE_CRUD_VERSION = hc_crud_caps = "=0.10.1"
NEW_CRUD_VERSION = hc_crud_caps = "0.10.2"

PRE_HDIE_VERSION = whi_hdi_extensions = "=0.3.0"
NEW_HDIE_VERSION = whi_hdi_extensions = "0.4"

PRE_HDKE_VERSION = whi_hdk_extensions = "=0.3.0"
NEW_HDKE_VERSION = whi_hdk_extensions = "0.4"

GG_REPLACE_LOCATIONS = ':(exclude)*.lock' devhub_sdk/Cargo.toml dnas/*/entry_types/Cargo.toml zomes/*/Cargo.toml

update-mere-memory-version:	reset-mere-memory
	git grep -l '$(PRE_MM_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_MM_VERSION)|$(NEW_MM_VERSION)|g'
update-crud-version:
	git grep -l '$(PRE_CRUD_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_CRUD_VERSION)|$(NEW_CRUD_VERSION)|g'
update-hdk-extensions-version:
	git grep -l '$(PRE_HDKE_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_HDKE_VERSION)|$(NEW_HDKE_VERSION)|g'
update-hdi-extensions-version:
	git grep -l '$(PRE_HDIE_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_HDIE_VERSION)|$(NEW_HDIE_VERSION)|g'


#
# Testing
#
TEST_UI			= tests/test.zip
TEST_HAPP		= tests/test.happ
TEST_WEBHAPP		= tests/test.webhapp

$(TEST_UI):
	dd if=/dev/zero of=$@ bs=1M count=1
$(TEST_HAPP):		$(ZOMEHUB_DNA)
	@echo "Packaging '$*': $@"
	@hc app pack -o $@ tests/test_happ/
$(TEST_WEBHAPP):	$(TEST_HAPP) $(TEST_UI)
	@echo "Packaging '$*': $@"
	@hc web-app pack -o $@ tests/test_webhapp/

test:
	make test-unit
	make test-integration
test-debug:
	make test-unit
	make test-integration-debug

test-unit:
	cd zomes; RUST_BACKTRACE=1 cargo test zome_hub -- --nocapture
	cd zomes; RUST_BACKTRACE=1 cargo test zome_hub_csr -- --nocapture
	cd zomes; RUST_BACKTRACE=1 cargo test dna_hub -- --nocapture
	cd zomes; RUST_BACKTRACE=1 cargo test dna_hub_csr -- --nocapture
	cd zomes; RUST_BACKTRACE=1 cargo test app_hub -- --nocapture
	cd zomes; RUST_BACKTRACE=1 cargo test app_hub_csr -- --nocapture

test-integration:
	make test-zome-hub-integration
	make test-dna-hub-integration
	make test-app-hub-integration
	make test-webapp-integration
test-integration-debug:
	make test-zome-hub-integration-debug
	make test-dna-hub-integration-debug
	make test-app-hub-integration-debug
	make test-webapp-integration-debug

test-zome-hub-integration:		$(ZOMEHUB_DNA)
	cd tests; LOG_LEVEL=warn npx mocha ./integration/test_zome_hub.js
test-zome-hub-integration-debug:	$(ZOMEHUB_DNA)
	cd tests; LOG_LEVEL=trace npx mocha ./integration/test_zome_hub.js

test-dna-hub-integration:		$(ZOMEHUB_DNA) $(DNAHUB_DNA)
	cd tests; LOG_LEVEL=warn npx mocha ./integration/test_dna_hub.js
test-dna-hub-integration-debug:		$(ZOMEHUB_DNA) $(DNAHUB_DNA)
	cd tests; LOG_LEVEL=trace npx mocha ./integration/test_dna_hub.js

test-app-hub-integration:		$(DEVHUB_HAPP) $(TEST_WEBHAPP)
	cd tests; LOG_LEVEL=warn npx mocha ./integration/test_app_hub.js
test-app-hub-integration-debug:		$(DEVHUB_HAPP) $(TEST_WEBHAPP)
	cd tests; LOG_LEVEL=trace npx mocha ./integration/test_app_hub.js

test-webapp-integration:		$(DEVHUB_HAPP) $(TEST_WEBHAPP)
	cd tests; LOG_LEVEL=warn npx mocha ./integration/test_webapp.js
test-webapp-integration-debug:		$(DEVHUB_HAPP) $(TEST_WEBHAPP)
	cd tests; LOG_LEVEL=trace npx mocha ./integration/test_webapp.js



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
