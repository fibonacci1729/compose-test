.PHONY: compose
compose: build-components
	wasm-tools compose -c config.yml -o target/wasm32-wasi/release/service.wasm target/wasm32-wasi/release/cors.wasm
	wasm-tools compose -c config.yml -o target/wasm32-wasi/release/service.wasm target/wasm32-wasi/release/auth.wasm

.PHONY: build-components
build-components: build-service build-middlewares

.PHONY: build-service
build-service:
	cd service && cargo component build --release

.PHONY: build-middlewares
build-middlewares: build-auth-middleware build-cors-middleware

.PHONY: build-auth-middleware
build-auth-middleware:
	cd middlewares/auth && cargo component build --release

.PHONY: build-cors-middleware
build-cors-middleware:
	cd middlewares/cors && cargo component build --release