
NAME			= zomehub

%/package-lock.json:	%/package.json
	touch $@
%/node_modules:		%/package-lock.json
	cd $*; npm install
	touch $@
fix-rust-compile-issue: # Force rebuild to fix rust issue (typically after dry-run)
	touch types/src/lib.rs
	touch sdk/src/lib.rs



#
# Types package
#
preview-types-crate:
	cargo publish -p hc_$(NAME)_types --dry-run --allow-dirty
	make fix-rust-compile-issue
publish-types-crate:
	cargo publish -p hc_$(NAME)_types



#
# SDK package
#
preview-sdk-crate:
	cargo publish -p hc_$(NAME)_sdk --dry-run --allow-dirty
	make fix-rust-compile-issue
publish-sdk-crate:
	cargo publish -p hc_$(NAME)_sdk



#
# Zomelets package
#
update-npm-package:
	cd zomelets; npm i --save $(PACKAGE)@
update-npm-all: \
	update-npm-bundles \
	update-npm-entities \
	update-npm-holo-hash \
	update-npm-mere-memory-zomelets \
	update-npm-zomelets
update-npm-bundles:
	PACKAGE=@spartan-hc/bundles make update-npm-package
update-npm-entities:
	PACKAGE=@spartan-hc/caps-entities make update-npm-package
update-npm-holo-hash:
	PACKAGE=@spartan-hc/holo-hash make update-npm-package
update-npm-mere-memory-zomelets:
	PACKAGE=@spartan-hc/mere-memory-zomelets make update-npm-package
update-npm-zomelets:
	PACKAGE=@spartan-hc/zomelets make update-npm-package

prepare-zomelets-package:	zomelets/node_modules
	cd zomelets; rm -f dist/*
	cd zomelets; npx webpack
	cd zomelets; MODE=production npx webpack
	cd zomelets; gzip -kf dist/*.js
preview-zomelets-package:	prepare-zomelets-package
	cd zomelets; npm pack --dry-run .
create-zomelets-package:	prepare-zomelets-package
	cd zomelets; npm pack .
publish-zomelets-package:	prepare-zomelets-package
	cd zomelets; npm publish --access public .
