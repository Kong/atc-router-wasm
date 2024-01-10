.PHONY: all setup build publish example clean

all: ;

setup:
	curl --proto '=https' --tlsv1.2 -sSf https://rustwasm.github.io/wasm-pack/installer/init.sh | sh

build:
ifeq (, $(shell which wasm-pack))
	$(error wasm-pack not found in PATH, consider installing it via "make setup")
endif
	wasm-pack build --scope kong --release --out-name atc-router
	# rename the package to "@kong/atc-router": wasm-pack does not support this yet
	sed -i '' -e 's#"name": "@kong/atc-router-wasm"#"name": "@kong/atc-router"#g' pkg/package.json

publish:
ifeq (, $(shell which npm))
	$(error "npm not found in PATH, consider installing it via https://docs.npmjs.com/downloading-and-installing-node-js-and-npm")
endif
	cd pkg && npm publish --access public

clean:
	rm -rf pkg
	rm -rf target
