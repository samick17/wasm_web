MAKEFLAGS += --silent

build:
	cd lib_wasm && \
		wasm-pack build --target web
	rm -rf web/src/libs
	cp -r lib_wasm/pkg web/src/libs
	cp web/src/libs/lib_wasm_bg.wasm web/public
	cd web && npm run build

launch:
	cd web && \
	serve -s public
