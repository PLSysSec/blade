LUCET_REPO=../lucet
LUCETC=$(LUCET_REPO)/target/debug/lucetc
LUCETC_FLAGS=--emit=so
WAT2WASM=$(HOME)/wabt-1.0.15/wat2wasm
AR=ar

.DEFAULT_GOAL=build

.PHONY: FORCE
FORCE:
$(LUCETC): FORCE
	cd $(LUCET_REPO) && cargo build

wasm_src/%.wasm: wasm_src/%.wat
	$(WAT2WASM) $< -o $@

wasm_obj/%_ref.so: wasm_src/%.wasm $(LUCETC)
	mkdir -p wasm_obj
	$(LUCETC) $(LUCETC_FLAGS) $< -o $@

target/debug/blade-benchmarks: wasm_obj/tea_ref.so wasm_obj/sha256_ref.so src
	cargo build

.PHONY: build
build: target/debug/blade-benchmarks

.PHONY: run
run: target/debug/blade-benchmarks
	cargo run

.PHONY: test
test: target/debug/blade-benchmarks
	cargo test

.PHONY: bench
bench: target/debug/blade-benchmarks
	cargo bench

.PHONY: disasm_tea
disasm_tea: wasm_obj/tea_ref.so
	objdump -SDg $< | less
.PHONY: disasm_sha256
disasm_sha256: wasm_obj/sha256_ref.so
	objdump -SDg $< | less

.PHONY: clean
clean:
	cargo clean
	-rm -rf wasm_obj

.PHONY: fullclean
fullclean: clean
	cd $(LUCET_REPO) && cargo clean
