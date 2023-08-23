.PHONY: all setup build publish clean

all: ;

setup:
ifeq (, $(shell which wasm-pack))
	$(info wasm-pack not found in PATH, installing)
	curl --proto '=https' --tlsv1.2 -sSf https://rustwasm.github.io/wasm-pack/installer/init.sh | sh
else
	$(info wasm-pack is already installed)
endif
	wasm-pack --version

build:
ifeq (, $(shell which wasm-packx))
	$(error wasm-pack not found in PATH, consider installing it via "make setup")
endif
	wasm-pack build --scope kong --release --out-name atc-router

publish:
ifeq (, $(shell which npm))
	$(error "npm not found in PATH, consider installing it via https://docs.npmjs.com/downloading-and-installing-node-js-and-npm")
endif
	cd pkg && \
	npm publish --access public

clean:
	rm -rf pkg
	rm -rf target
