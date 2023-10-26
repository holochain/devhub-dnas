
SHELL			= bash
NAME			= devhub

# External WASM dependencies
MERE_MEMORY_VERSION	= 0.91.0
MERE_MEMORY_WASM	= zomes/mere_memory.wasm
MERE_MEMORY_API_WASM	= zomes/mere_memory_api.wasm

# External DNA dependencies
PORTAL_VERSION		= 0.9.1
PORTAL_DNA		= dnas/portal.dna


# hApp
DEVHUB_HAPP		= happ/$(NAME).happ

# DNAs
ZOMEHUB_DNA		= dnas/zomehub.dna
DNAHUB_DNA		= dnas/dnahub.dna
APPHUB_DNA		= dnas/apphub.dna

# Integrity Zomes
ZOMEHUB_WASM		= zomes/zomehub.wasm
DNAHUB_WASM		= zomes/dnahub.wasm
APPHUB_WASM		= zomes/apphub.wasm

# Coordinator WASMs
ZOMEHUB_CSR_WASM	= zomes/zomehub_csr.wasm
DNAHUB_CSR_WASM		= zomes/dnahub_csr.wasm
APPHUB_CSR_WASM		= zomes/apphub_csr.wasm

TARGET			= release
TARGET_DIR		= target/wasm32-unknown-unknown/release
COMMON_SOURCE_FILES	= Makefile zomes/Cargo.toml
INT_SOURCE_FILES	= $(COMMON_SOURCE_FILES) \
				dnas/%/types/Cargo.toml dnas/%/types/src/*.rs \
				zomes/%/Cargo.toml zomes/%/src/*.rs
CSR_SOURCE_FILES	= $(COMMON_SOURCE_FILES) $(INT_SOURCE_FILES) \
				zomes/%_csr/Cargo.toml zomes/%_csr/src/*.rs \
				dnas/%/sdk/Cargo.toml dnas/%/sdk/src/*.rs \
				devhub_sdk/Cargo.toml devhub_sdk/src/*.rs


#
# Project
#
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


$(DEVHUB_HAPP):			$(ZOMEHUB_DNA) $(DNAHUB_DNA) $(APPHUB_DNA) $(PORTAL_DNA) happ/happ.yaml
	hc app pack -o $@ ./happ/

$(ZOMEHUB_DNA):			$(ZOMEHUB_WASM) $(ZOMEHUB_CSR_WASM) $(MERE_MEMORY_WASM) $(MERE_MEMORY_API_WASM)
$(DNAHUB_DNA):			$(DNAHUB_WASM) $(DNAHUB_CSR_WASM)
$(APPHUB_DNA):			$(APPHUB_WASM) $(APPHUB_CSR_WASM)

dnas/%.dna:			dnas/%/dna.yaml
	@echo "Packaging '$*': $@"
	@hc dna pack -o $@ dnas/$*

zomes/%.wasm:			$(TARGET_DIR)/%.wasm
	@echo -e "\x1b[38;2mCopying WASM ($<) to 'zomes' directory: $@\x1b[0m"; \
	cp $< $@

$(TARGET_DIR)/%.wasm:		$(INT_SOURCE_FILES)
	rm -f zomes/$*.wasm
	@echo -e "\x1b[37mBuilding zome '$*' -> $@\x1b[0m";
	cd zomes; \
	RUST_BACKTRACE=1 cargo build --release \
	    --target wasm32-unknown-unknown \
	    --package $*
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time
$(TARGET_DIR)/%_csr.wasm:	$(CSR_SOURCE_FILES)
	rm -f zomes/$*_csr.wasm
	@echo -e "\x1b[37mBuilding zome '$*_csr' -> $@\x1b[0m";
	cd zomes; \
	RUST_BACKTRACE=1 cargo build --release \
	    --target wasm32-unknown-unknown \
	    --package $*_csr
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time

$(PORTAL_DNA):
	wget -O $@ "https://github.com/holochain/portal-dna/releases/download/v$(PORTAL_VERSION)/portal.dna" || rm -f $(PORTAL_DNA)

$(MERE_MEMORY_WASM):
	curl --fail -L "https://github.com/mjbrisebois/hc-zome-mere-memory/releases/download/v$(MERE_MEMORY_VERSION)/mere_memory.wasm" --output $@
$(MERE_MEMORY_API_WASM):
	curl --fail -L "https://github.com/mjbrisebois/hc-zome-mere-memory/releases/download/v$(MERE_MEMORY_VERSION)/mere_memory_api.wasm" --output $@

reset-mere-memory:
	rm zomes/mere_memory*.wasm
	make $(MERE_MEMORY_WASM) $(MERE_MEMORY_API_WASM)

PRE_MM_VERSION = mere_memory_types = "0.90.0"
NEW_MM_VERSION = mere_memory_types = "0.91.0"

PRE_CRUD_VERSION = hc_crud_caps = "=0.10.1"
NEW_CRUD_VERSION = hc_crud_caps = "0.10.2"

PRE_HDIE_VERSION = whi_hdi_extensions = "=0.3.0"
NEW_HDIE_VERSION = whi_hdi_extensions = "0.4"

PRE_HDKE_VERSION = whi_hdk_extensions = "=0.3.0"
NEW_HDKE_VERSION = whi_hdk_extensions = "0.4"

PRE_PSDK_VERSION = hc_portal_sdk = "0.1.1"
NEW_PSDK_VERSION = hc_portal_sdk = "0.1.2"

GG_REPLACE_LOCATIONS = ':(exclude)*.lock' devhub_sdk/Cargo.toml dnas/*/types/Cargo.toml dnas/*/sdk/Cargo.toml zomes/*/Cargo.toml

update-mere-memory-version:	reset-mere-memory
	git grep -l '$(PRE_MM_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_MM_VERSION)|$(NEW_MM_VERSION)|g'
update-crud-version:
	git grep -l '$(PRE_CRUD_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_CRUD_VERSION)|$(NEW_CRUD_VERSION)|g'
update-hdk-extensions-version:
	git grep -l '$(PRE_HDKE_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_HDKE_VERSION)|$(NEW_HDKE_VERSION)|g'
update-hdi-extensions-version:
	git grep -l '$(PRE_HDIE_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_HDIE_VERSION)|$(NEW_HDIE_VERSION)|g'
update-portal-sdk-version:
	git grep -l '$(PRE_PSDK_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_PSDK_VERSION)|$(NEW_PSDK_VERSION)|g'

npm-reinstall-local:
	cd tests; npm uninstall $(NPM_PACKAGE); npm i --save $(LOCAL_PATH)
npm-reinstall-public:
	cd tests; npm uninstall $(NPM_PACKAGE); npm i --save $(NPM_PACKAGE)
npm-use-app-interface-client-public:
npm-use-app-interface-client-local:
npm-use-app-interface-client-%:
	NPM_PACKAGE=@spartan-hc/app-interface-client LOCAL_PATH=../../app-interface-client-js make npm-reinstall-$*



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

%/package-lock.json:	%/package.json
	touch $@
%/node_modules:		%/package-lock.json
	cd $*; npm install
	touch $@
test-setup:		tests/node_modules \
			dnas/zomehub/zomelets/node_modules \
			dnas/dnahub/zomelets/node_modules \
			dnas/apphub/zomelets/node_modules

test:
	make test-unit
	make test-integration
	make test-real-uploads
test-debug:
	make test-unit-debug
	make test-integration-debug
	make test-real-uploads-debug

# Unit tests
test-crate:
	cd $(SRC); CARGO_TARGET_DIR=../target cargo test --quiet --tests
test-crate-debug:
	cd $(SRC); RUST_BACKTRACE=1 CARGO_TARGET_DIR=../target cargo test -- --nocapture --show-output
test-unit:
	SRC=zomes make test-crate
	make test-zomehub-unit
	make test-dnahub-unit
	make test-apphub-unit
# ISSUE: for some reason these break after wasm has been built (fix 'rm -r target')
test-unit-debug:
	SRC=zomes make test-crate-debug
	make test-zomehub-unit-debug
	make test-dnahub-unit-debug
	make test-apphub-unit-debug

test-%hub-unit:
	SRC=dnas/$*hub make test-crate
test-%hub-unit-debug:
	SRC=dnas/$*hub make test-crate-debug

test-zome-unit-%:
	cd zomes; cargo test -p $* --quiet
test-zome-unit-%-debug:
	cd zomes; RUST_BACKTRACE=1 cargo test -p $* -- --nocapture --show-output

# Integration tests
test-integration:
	make test-zomehub
	make test-dnahub
	make test-apphub
	make test-webapp-upload
test-integration-debug:
	make test-zomehub-debug
	make test-dnahub-debug
	make test-apphub-debug
	make test-webapp-upload-debug

test-zomehub:				test-setup $(ZOMEHUB_DNA)
	cd tests; LOG_LEVEL=warn npx mocha ./integration/test_zomehub.js
test-zomehub-debug:			test-setup $(ZOMEHUB_DNA)
	cd tests; LOG_LEVEL=trace npx mocha ./integration/test_zomehub.js

test-dnahub:				test-setup $(DNAHUB_DNA) $(ZOMEHUB_DNA)
	cd tests; LOG_LEVEL=warn npx mocha ./integration/test_dnahub.js
test-dnahub-debug:			test-setup $(DNAHUB_DNA) $(ZOMEHUB_DNA)
	cd tests; LOG_LEVEL=trace npx mocha ./integration/test_dnahub.js

test-apphub:				test-setup $(APPHUB_DNA) $(DNAHUB_DNA) $(ZOMEHUB_DNA)
	cd tests; LOG_LEVEL=warn npx mocha ./integration/test_apphub.js
test-apphub-debug:			test-setup $(APPHUB_DNA) $(DNAHUB_DNA) $(ZOMEHUB_DNA)
	cd tests; LOG_LEVEL=trace npx mocha ./integration/test_apphub.js

test-webapp-upload:			test-setup $(TEST_WEBHAPP) $(DEVHUB_HAPP)
	cd tests; LOG_LEVEL=warn npx mocha ./integration/test_webapp_upload.js
test-webapp-upload-debug:		test-setup $(TEST_WEBHAPP) $(DEVHUB_HAPP)
	cd tests; LOG_LEVEL=trace npx mocha ./integration/test_webapp_upload.js

# Real-input tests
test-real-uploads:
	make test-real-zome-upload
	make test-real-dna-upload
	make test-real-app-upload
test-real-uploads-debug:
	make test-real-zome-upload-debug
	make test-real-dna-upload-debug
	make test-real-app-upload-debug

test-real-zome-upload:			test-setup $(ZOMEHUB_DNA)
	cd tests; LOG_LEVEL=warn npx mocha ./integration/test_real_zome_upload.js
test-real-zome-upload-debug:		test-setup $(ZOMEHUB_DNA)
	cd tests; LOG_LEVEL=trace npx mocha ./integration/test_real_zome_upload.js

test-real-dna-upload:			test-setup $(ZOMEHUB_DNA) $(DNAHUB_DNA)
	cd tests; LOG_LEVEL=warn npx mocha ./integration/test_real_dna_upload.js
test-real-dna-upload-debug:		test-setup $(ZOMEHUB_DNA) $(DNAHUB_DNA)
	cd tests; LOG_LEVEL=trace npx mocha ./integration/test_real_dna_upload.js

test-real-app-upload:			test-setup $(DEVHUB_HAPP)
	cd tests; LOG_LEVEL=warn npx mocha ./integration/test_real_app_upload.js
test-real-app-upload-debug:		test-setup $(DEVHUB_HAPP)
	cd tests; LOG_LEVEL=trace npx mocha ./integration/test_real_app_upload.js



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
