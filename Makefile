
SHELL			= bash

NAME			= devhub

HAPP_BUNDLE		= devhub.happ
DNAREPO			= bundled/dnarepo.dna
HAPPDNA			= bundled/happs.dna
ASSETSDNA		= bundled/web_assets.dna

PORTAL_DNA		= bundled/portal.dna

TARGET			= release

# Integrity WASMs
DNAREPO_CORE		= zomes/dnarepo_core.wasm
HAPPS_CORE		= zomes/happs_core.wasm
WEB_ASSETS_CORE		= zomes/web_assets_core.wasm

# Coordination WASMs
DNA_LIBRARY_WASM	= zomes/dna_library.wasm
HAPP_LIBRARY_WASM	= zomes/happ_library.wasm
REVIEWS_WASM		= zomes/reviews.wasm
WEB_ASSETS_WASM		= zomes/web_assets.wasm

# External WASM dependencies
MERE_MEMORY_WASM	= zomes/mere_memory.wasm
MERE_MEMORY_API_WASM	= zomes/mere_memory_api.wasm


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
	    zomes/target \
	    $(HAPP_BUNDLE) \
	    $(DNAREPO) $(HAPPDNA) $(ASSETSDNA) \
	    $(DNA_LIBRARY_WASM) $(REVIEWS_WASM) $(HAPP_LIBRARY_WASM) $(WEB_ASSETS_WASM) $(MERE_MEMORY_API_WASM)

rebuild:			clean build
build:				$(HAPP_BUNDLE)


$(HAPP_BUNDLE):			$(DNAREPO) $(HAPPDNA) $(ASSETSDNA) $(PORTAL_DNA) bundled/happ.yaml
	hc app pack -o $@ ./bundled/

$(DNAREPO):			$(DNAREPO_CORE) $(DNA_LIBRARY_WASM) $(REVIEWS_WASM) $(MERE_MEMORY_API_WASM) $(MERE_MEMORY_WASM)
$(HAPPDNA):			$(HAPPS_CORE) $(HAPP_LIBRARY_WASM)
$(ASSETSDNA):			$(WEB_ASSETS_CORE) $(WEB_ASSETS_WASM)

# bundled/happs/dna.yaml:		$(DNAREPO) #$(ASSETSDNA)
# 	node tests/update_happ_dna_yaml.js
bundled/%.dna:			bundled/%/dna.yaml
	@echo "Packaging '$*': $@"
	@hc dna pack -o $@ bundled/$*

zomes/%.wasm:			zomes/target/wasm32-unknown-unknown/release/%.wasm
	cp $< $@
zomes/target/wasm32-unknown-unknown/release/%.wasm:	Makefile devhub_types/src/*.rs devhub_types/Cargo.toml zomes/%/src/*.rs zomes/%/Cargo.toml zomes/%/Cargo.lock
	@echo "Building  '$*' WASM: $@"; \
	cd zomes; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build --release \
	    --target wasm32-unknown-unknown \
	    --package $*
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time
zomes/%/Cargo.lock:
	touch $@

$(MERE_MEMORY_WASM):
	curl --fail -L "https://github.com/mjbrisebois/hc-zome-mere-memory/releases/download/v$$(echo $(NEW_MM_VERSION))/mere_memory.wasm" --output $@
$(MERE_MEMORY_API_WASM):
	curl --fail -L "https://github.com/mjbrisebois/hc-zome-mere-memory/releases/download/v$$(echo $(NEW_MM_VERSION))/mere_memory_api.wasm" --output $@

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

$(PORTAL_DNA):
	wget -O $@ "https://github.com/holochain/portal-dna/releases/download/v$(NEW_PORTAL_VERSION)/portal.dna" || rm -f $(PORTAL_DNA)
copy-portal-from-local:
	cp ../app-store-dnas/bundled/portal.dna $(PORTAL_DNA)



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

tests/test.dna:
	cp $(DNAREPO) $@
tests/test.gz:
	gzip -kc $(DNAREPO) > $@

# DNAs
test-setup:			tests/node_modules

test-dnas:			test-setup test-general		test-dnarepo		test-happs		test-webassets		test-multi		test-reviews
test-dnas-debug:		test-setup test-general-debug	test-dnarepo-debug	test-happs-debug	test-webassets-debug	test-multi-debug	test-reviews-debug

test-general:			test-setup $(DNAREPO)
	cd tests; RUST_LOG=none LOG_LEVEL=fatal npx mocha integration/test_generic_fns.js
test-general-debug:		test-setup $(DNAREPO)
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_generic_fns.js

test-dnarepo:			test-setup $(DNAREPO)
	cd tests; RUST_LOG=none LOG_LEVEL=fatal npx mocha integration/test_dnarepo.js
test-dnarepo-debug:		test-setup $(DNAREPO)
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_dnarepo.js

test-reviews:			test-setup $(DNAREPO)
	cd tests; RUST_LOG=none LOG_LEVEL=fatal npx mocha integration/test_reviews.js
test-reviews-debug:		test-setup $(DNAREPO)
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_reviews.js

test-happs:			test-setup $(HAPPDNA)
	cd tests; RUST_LOG=none LOG_LEVEL=fatal npx mocha integration/test_happs.js
test-happs-debug:		test-setup $(HAPPDNA)
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_happs.js

test-guis:			test-setup $(HAPPDNA) $(ASSETSDNA)
	cd tests; RUST_LOG=none LOG_LEVEL=fatal npx mocha integration/test_guis.js
test-guis-debug:		test-setup $(HAPPDNA) $(ASSETSDNA)
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_guis.js

test-webassets:			test-setup $(ASSETSDNA) tests/test.gz
	cd tests; RUST_LOG=none LOG_LEVEL=fatal npx mocha integration/test_webassets.js
test-webassets-debug:		test-setup $(ASSETSDNA) tests/test.gz
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_webassets.js

test-multi:			test-setup $(DNAREPO) $(HAPPDNA) $(ASSETSDNA) tests/test.gz tests/test.dna
	cd tests; RUST_LOG=none LOG_LEVEL=fatal npx mocha integration/test_multiple.js
test-multi-debug:		test-setup $(DNAREPO) $(HAPPDNA) $(ASSETSDNA) tests/test.gz tests/test.dna
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_multiple.js


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

PRE_HDK_VERSION = "0.3.0-beta-dev.7"
NEW_HDK_VERSION = "0.2.1-beta-rc.0"

PRE_HDI_VERSION = "0.4.0-beta-dev.5"
NEW_HDI_VERSION = "0.3.1-beta-rc.0"

PRE_CRUD_VERSION = "0.80.0"
NEW_CRUD_VERSION = "0.8.0"

PRE_MM_VERSION = "0.86.0"
NEW_MM_VERSION = "0.87.0"

# PRE_PORTAL_VERSION = "0.3.0"
NEW_PORTAL_VERSION = "0.7.0"

GG_REPLACE_LOCATIONS = ':(exclude)*.lock' devhub_types/ zomes/*/

update-hdk-version:
	git grep -l '$(PRE_HDK_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_HDK_VERSION)|$(NEW_HDK_VERSION)|g'
update-hdi-version:
	git grep -l '$(PRE_HDI_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_HDI_VERSION)|$(NEW_HDI_VERSION)|g'
update-crud-version:
	git grep -l '$(PRE_CRUD_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_CRUD_VERSION)|$(NEW_CRUD_VERSION)|g'
update-mere-memory-version:
	git grep -l '$(PRE_MM_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_MM_VERSION)|$(NEW_MM_VERSION)|g'
	rm -f zomes/mere_memory*.wasm
update-portal-version:
	rm -f $(PORTAL_DNA)
