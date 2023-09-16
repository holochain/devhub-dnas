
SHELL			= bash

NAME			= devhub

HAPP_BUNDLE		= $(NAME).happ
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
	cp ../zome-mere-memory/target/wasm32-unknown-unknown/release/mere_memory.wasm $@
$(MERE_MEMORY_API_WASM):
	cp ../zome-mere-memory/target/wasm32-unknown-unknown/release/mere_memory_api.wasm $@

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
	cp ../portal-dna/bundled/portal.dna $@
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

PRE_HDK_VERSION = "0.1.3-beta-rc.1"
NEW_HDK_VERSION = "0.1.4"

PRE_HDI_VERSION = "0.2.3-beta-rc.0"
NEW_HDI_VERSION = "0.2.4"

PRE_CRUD_VERSION = rev = "1f4295a79e93438474c4cf7a8d304e4143a4b94f"
NEW_CRUD_VERSION = rev = "91438a2e167b5bfea3c6673fe2f1da827c7da2c9"

GG_REPLACE_LOCATIONS = ':(exclude)*.lock' devhub_types/ zomes/*/

update-hdk-version:
	git grep -l '$(PRE_HDK_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_HDK_VERSION)|$(NEW_HDK_VERSION)|g'
update-hdi-version:
	git grep -l '$(PRE_HDI_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_HDI_VERSION)|$(NEW_HDI_VERSION)|g'
update-crud-version:
	git grep -l '$(PRE_CRUD_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|$(PRE_CRUD_VERSION)|$(NEW_CRUD_VERSION)|g'
update-mere-memory-version:
	rm zomes/mere_memory*.wasm
update-portal-version:
	rm -f $(PORTAL_DNA)
