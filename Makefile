.PHONY:			FORCE
SHELL			= bash
NAME			= devhub

# External WASM dependencies
MERE_MEMORY_WASM	= .devhub/zomes/@spartan-hc/mere_memory.wasm
MERE_MEMORY_API_WASM	= .devhub/zomes/@spartan-hc/mere_memory_csr.wasm

COOP_CONTENT_WASM	= .devhub/zomes/@spartan-hc/coop_content.wasm
COOP_CONTENT_CSR_WASM	= .devhub/zomes/@spartan-hc/coop_content_csr.wasm

# External DNA dependencies
PORTAL_VERSION		= 0.17.0
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
				zomes/%/Cargo.toml zomes/%/src/*.rs \
				zomes/%/src/**
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
	    .cargo target \
	    $(DEVHUB_HAPP) \
	    $(ZOMEHUB_DNA) $(DNAHUB_DNA) $(APPHUB_DNA) \
	    $(ZOMEHUB_WASM) $(ZOMEHUB_CSR_WASM) \
	    $(DNAHUB_WASM) $(DNAHUB_CSR_WASM) \
	    $(APPHUB_WASM) $(APPHUB_CSR_WASM) \
	    $(MERE_MEMORY_WASM) $(MERE_MEMORY_API_WASM) \
	    $(COOP_CONTENT_WASM) $(COOP_CONTENT_CSR_WASM)

rebuild:			clean build
build:				$(DEVHUB_HAPP)


$(DEVHUB_HAPP):			$(ZOMEHUB_DNA) $(DNAHUB_DNA) $(APPHUB_DNA) $(PORTAL_DNA) happ/happ.yaml
	hc app pack -o $@ ./happ/

$(ZOMEHUB_DNA):			$(ZOMEHUB_WASM) $(ZOMEHUB_CSR_WASM)		\
				$(MERE_MEMORY_WASM) $(MERE_MEMORY_API_WASM)	\
				$(COOP_CONTENT_WASM) $(COOP_CONTENT_CSR_WASM)
$(DNAHUB_DNA):			$(DNAHUB_WASM) $(DNAHUB_CSR_WASM)
$(APPHUB_DNA):			$(APPHUB_WASM) $(APPHUB_CSR_WASM)

dnas/%.dna:			dnas/%/dna.yaml
	@echo "Packaging '$*': $@"
	@hc dna pack -o $@ dnas/$*

$(MERE_MEMORY_WASM):
$(MERE_MEMORY_API_WASM):
$(COOP_CONTENT_WASM):
$(COOP_CONTENT_CSR_WASM):
.devhub/zomes/%.wasm:
	@if [[ -e "$@" ]]; then \
		touch $@;	\
	else			\
		echo -e "\x1b[32;2mInstall WASM dependency '$*' using devhub (https://github.com/holochain/devhub-cli)\x1b[0m"; \
		exit 1;		\
	fi
zomes/%.wasm:			$(TARGET_DIR)/%.wasm
	@echo -e "\x1b[38;2mCopying WASM ($<) to 'zomes' directory: $@\x1b[0m"; \
	cp $< $@

$(TARGET_DIR)/%.wasm:		$(INT_SOURCE_FILES)
	rm -f zomes/$*.wasm
	@echo -e "\x1b[37mBuilding zome '$*' -> $@\x1b[0m";
	RUST_BACKTRACE=1 cargo build --release \
	    --target wasm32-unknown-unknown \
	    --package $*
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time
$(TARGET_DIR)/%_csr.wasm:	$(CSR_SOURCE_FILES)
	rm -f zomes/$*_csr.wasm
	@echo -e "\x1b[37mBuilding zome '$*_csr' -> $@\x1b[0m";
	RUST_BACKTRACE=1 cargo build --release \
	    --target wasm32-unknown-unknown \
	    --package $*_csr
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time

$(PORTAL_DNA):
	wget -O $@ "https://github.com/holochain/portal-dna/releases/download/v$(PORTAL_VERSION)/portal.dna" || rm -f $(PORTAL_DNA)

reset-mere-memory:
	rm -f zomes/mere_memory*.wasm
	make $(MERE_MEMORY_WASM) $(MERE_MEMORY_API_WASM)
reset-portal:
	rm -f dnas/portal.dna
	make $(PORTAL_DNA)

PRE_MM_VERSION = mere_memory_types = "0.97.0"
NEW_MM_VERSION = mere_memory_types = "0.98.0"

PRE_CRUD_VERSION = hc_crud_caps = "0.17"
NEW_CRUD_VERSION = hc_crud_caps = "0.18"

PRE_HDIE_VERSION = whi_hdi_extensions = "0.12"
NEW_HDIE_VERSION = whi_hdi_extensions = "0.13"

PRE_HDKE_VERSION = whi_hdk_extensions = "0.12"
NEW_HDKE_VERSION = whi_hdk_extensions = "0.13"

PRE_PSDK_VERSION = hc_portal_sdk = "0.8"
NEW_PSDK_VERSION = hc_portal_sdk = "0.9"

PRE_HIT_VERSION = holochain_integrity_types = "=0.4.0-dev.12"
NEW_HIT_VERSION = holochain_integrity_types = "=0.4.0-dev.15"

PRE_HZT_VERSION = holochain_zome_types = { version = "=0.4.0-dev.14"
NEW_HZT_VERSION = holochain_zome_types = { version = "=0.4.0-dev.18"

GG_REPLACE_LOCATIONS = ':(exclude)*.lock' Cargo.toml devhub_sdk/Cargo.toml dnas/*/types/Cargo.toml dnas/*/sdk/Cargo.toml zomes/*/Cargo.toml

update-mere-memory-version:
	git grep -l '$(PRE_MM_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_MM_VERSION)|$(NEW_MM_VERSION)|g'
update-crud-version:
	git grep -l '$(PRE_CRUD_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_CRUD_VERSION)|$(NEW_CRUD_VERSION)|g'
update-hdk-extensions-version:
	git grep -l '$(PRE_HDKE_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_HDKE_VERSION)|$(NEW_HDKE_VERSION)|g'
update-hdi-extensions-version:
	git grep -l '$(PRE_HDIE_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_HDIE_VERSION)|$(NEW_HDIE_VERSION)|g'
update-portal-sdk-version:	reset-portal
	git grep -l '$(PRE_PSDK_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_PSDK_VERSION)|$(NEW_PSDK_VERSION)|g'
update-integrity-types-version:
	git grep -l '$(PRE_HIT_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_HIT_VERSION)|$(NEW_HIT_VERSION)|g'
update-zome-types-version:
	git grep -l '$(PRE_HZT_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_HZT_VERSION)|$(NEW_HZT_VERSION)|g'
update-edition:
	git grep -l '$(PRE_EDITION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's/$(PRE_EDITION)/$(NEW_EDITION)/g'

npm-reinstall-local:
	cd tests; npm uninstall $(NPM_PACKAGE); npm i --save $(LOCAL_PATH)
npm-reinstall-public:
	cd tests; npm uninstall $(NPM_PACKAGE); npm i --save $(NPM_PACKAGE)
npm-reinstall-%-zomelets-local:
	cd dnas/$*/zomelets; npm uninstall $(NPM_PACKAGE); npm i --save $(LOCAL_PATH)
npm-reinstall-%-zomelets-public:
	cd dnas/$*/zomelets; npm uninstall $(NPM_PACKAGE); npm i --save $(NPM_PACKAGE)

npm-use-app-interface-client-public:
npm-use-app-interface-client-local:
npm-use-app-interface-client-%:
	NPM_PACKAGE=@spartan-hc/app-interface-client LOCAL_PATH=../../app-interface-client-js make npm-reinstall-$*

npm-use-backdrop-public:
npm-use-backdrop-local:
npm-use-backdrop-%:
	NPM_PACKAGE=@spartan-hc/holochain-backdrop LOCAL_PATH=../../node-backdrop make npm-reinstall-$*

npm-use-bundles-public:
npm-use-bundles-local:
npm-use-bundles-%:
	NPM_PACKAGE=@spartan-hc/bundles LOCAL_PATH=../../bundles-js make npm-reinstall-$*
	NPM_PACKAGE=@spartan-hc/bundles LOCAL_PATH=../../../../bundles-js make npm-reinstall-zomehub-zomelets-$*
	NPM_PACKAGE=@spartan-hc/bundles LOCAL_PATH=../../../../bundles-js make npm-reinstall-dnahub-zomelets-$*
	NPM_PACKAGE=@spartan-hc/bundles LOCAL_PATH=../../../../bundles-js make npm-reinstall-apphub-zomelets-$*



#
# Testing
#
TEST_UI			= tests/test.zip
TEST_HAPP		= tests/test.happ
TEST_WEBHAPP		= tests/test.webhapp

$(TEST_UI):
	dd if=/dev/zero of=$@ bs=1M count=1
$(TEST_HAPP):		$(ZOMEHUB_DNA)
	@echo "Packaging: $@"
	@hc app pack -o $@ tests/test_happ/
$(TEST_WEBHAPP):	$(TEST_HAPP) $(TEST_UI)
	@echo "Packaging: $@"
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

# Unit tests
CRATE_DEBUG_LEVELS	= normal info debug trace
test-crate:
	@if [[ "$(CRATE_DEBUG_LEVELS)" == *"$(DEBUG_LEVEL)"* ]]; then \
		RUST_BACKTRACE=1 cargo test -- --nocapture --show-output; \
	else \
		cargo test --quiet --tests; \
	fi
test-unit:
	make test-crate

# Integration tests
DEBUG_LEVEL	       ?= warn
TEST_ENV_VARS		= LOG_LEVEL=$(DEBUG_LEVEL)
MOCHA_OPTS		= -n enable-source-maps -t 10000

test-integration:
	make test-zomehub
	make test-dnahub
	make test-apphub
	make test-real-uploads
	make test-webapp-upload

test-zomehub:				test-setup $(ZOMEHUB_DNA)
	cd tests; $(TEST_ENV_VARS) npx mocha $(MOCHA_OPTS) ./integration/test_zomehub.js
test-dnahub:				test-setup $(ZOMEHUB_DNA) $(DNAHUB_DNA)
	cd tests; $(TEST_ENV_VARS) npx mocha $(MOCHA_OPTS) ./integration/test_dnahub.js
test-apphub:				test-setup $(ZOMEHUB_DNA) $(DNAHUB_DNA) $(APPHUB_DNA)
	cd tests; $(TEST_ENV_VARS) npx mocha $(MOCHA_OPTS) ./integration/test_apphub.js
test-webapp-upload:			test-setup $(TEST_WEBHAPP) $(DEVHUB_HAPP)
	cd tests; $(TEST_ENV_VARS) npx mocha $(MOCHA_OPTS) ./integration/test_webapp_upload.js

test-integration-zomehub-group-management:	test-setup $(ZOMEHUB_DNA)
	cd tests; $(TEST_ENV_VARS) npx mocha $(MOCHA_OPTS) ./integration/test_zomehub_group_management.js

# Real-input tests
test-real-uploads:
	make test-real-zome-upload
	make test-real-dna-upload
	make test-real-app-upload

test-real-zome-upload:			test-setup $(ZOMEHUB_DNA)
	cd tests; $(TEST_ENV_VARS) npx mocha $(MOCHA_OPTS) ./integration/test_real_zome_upload.js
test-real-dna-upload:			test-setup $(ZOMEHUB_DNA) $(DNAHUB_DNA)
	cd tests; $(TEST_ENV_VARS) npx mocha $(MOCHA_OPTS) ./integration/test_real_dna_upload.js
test-real-app-upload:			test-setup $(DEVHUB_HAPP)
	cd tests; $(TEST_ENV_VARS) npx mocha $(MOCHA_OPTS) ./integration/test_real_app_upload.js



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



#
# DevHub SDK package
#
.cargo/credentials:
	cp ~/$@ $@
fix-rust-compile-issue: # Force rebuild to fix rust issue (typically after dry-run)
	touch devhub_sdk/src/lib.rs
	touch dnas/*/types/src/lib.rs
	touch zomes/*/src/lib.rs
preview-sdk-crate:		test .cargo/credentials
	cd devhub_sdk; cargo publish --dry-run --allow-dirty
	make fix-rust-compile-issue
publish-sdk-crate:		test .cargo/credentials
	cd devhub_sdk; cargo publish



#
# Previewing DNA Packages
#
preview-%-packages: 		preview-%-types-crate \
				preview-%-sdk-crate \
				preview-%-zomelets-package
	@echo -e "\x1b[37mFinished previewing packages for '$*'\x1b[0m";

preview-zomehub-packages:
preview-dnahub-packages:
preview-apphub-packages:



#
# Publishing Types Packages
#
preview-%-types-crate:		 test-unit test-% .cargo/credentials
	cd dnas/$*; make preview-types-crate
publish-%-types-crate:		 test-unit test-% .cargo/credentials
	cd dnas/$*; make publish-types-crate

preview-zomehub-types-crate:
publish-zomehub-types-crate:

preview-dnahub-types-crate:
publish-dnahub-types-crate:

preview-apphub-types-crate:
publish-apphub-types-crate:



#
# Publishing SDK Packages
#
preview-%-sdk-crate:		 test-unit test-% .cargo/credentials
	cd dnas/$*; make preview-sdk-crate
publish-%-sdk-crate:		 test-unit test-% .cargo/credentials
	cd dnas/$*; make publish-sdk-crate

preview-zomehub-sdk-crate:
publish-zomehub-sdk-crate:

preview-dnahub-sdk-crate:
publish-dnahub-sdk-crate:

preview-apphub-sdk-crate:
publish-apphub-sdk-crate:



#
# Publishing Zomelets Packages
#
prepare-%-zomelets-package:	zomelets/node_modules
	cd dnas/$*; make prepare-zomelets-package
preview-%-zomelets-package:	clean-files test-unit test-%
	cd dnas/$*; make preview-zomelets-package
create-%-zomelets-package:	clean-files test-unit test-%
	cd dnas/$*; make create-zomelets-package
publish-%-zomelets-package:	clean-files test-unit test-%
	cd dnas/$*; make publish-zomelets-package

prepare-zomehub-zomelets-package:
preview-zomehub-zomelets-package:
create-zomehub-zomelets-package:
publish-zomehub-zomelets-package:

prepare-dnahub-zomelets-package:
preview-dnahub-zomelets-package:
create-dnahub-zomelets-package:
publish-dnahub-zomelets-package:

prepare-apphub-zomelets-package:
preview-apphub-zomelets-package:
create-apphub-zomelets-package:
publish-apphub-zomelets-package:
