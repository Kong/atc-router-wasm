.PHONY: all setup build publish clean

all: ;

setup:
	curl --proto '=https' --tlsv1.2 -sSf https://rustwasm.github.io/wasm-pack/installer/init.sh | sh

build:
ifeq (, $(shell which wasm-pack))
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
