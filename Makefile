WASMTIME_REPO=../wasmtime
WASMTIME=$(WASMTIME_REPO)/target/debug/wasmtime
WAT2WASM=wat2wasm
AR=ar

.DEFAULT_GOAL=build

.PHONY: FORCE
FORCE:
$(WASMTIME): FORCE
	cd $(WASMTIME_REPO) && cargo build

wasm_src/%.wasm: wasm_src/%.wat
	$(WAT2WASM) $< -o $@

wasm_obj/%_ref.o: wasm_src/%.wasm $(WASMTIME)
	mkdir -p wasm_obj
	$(WASMTIME) wasm2obj $< $@

wasm_obj/lib%_ref.a: wasm_obj/%_ref.o
	$(AR) rcs $@ $<

.PHONY: build
build: wasm_obj/libtea_ref.a src
	cargo build

.PHONY: run
run: build
	cargo run

.PHONY: clean
clean:
	cargo clean
	-rm wasm_obj/*

.PHONY: fullclean
fullclean: clean
	cd $(WASMTIME_REPO) && cargo clean
