LUCET_REPO=../lucet
LUCETC=$(LUCET_REPO)/target/debug/lucetc
LUCETC_FLAGS=--emit=obj
WAT2WASM=$(HOME)/wabt-1.0.15/wat2wasm
AR=ar

.DEFAULT_GOAL=build

.PHONY: FORCE
FORCE:
$(LUCETC): FORCE
	cd $(LUCET_REPO) && cargo build

wasm_src/%.wasm: wasm_src/%.wat
	$(WAT2WASM) $< -o $@

wasm_obj/%_ref.o: wasm_src/%.wasm $(LUCETC)
	mkdir -p wasm_obj
	$(LUCETC) $(LUCETC_FLAGS) $< -o $@

wasm_obj/lib%_ref.a: wasm_obj/%_ref.o
	$(AR) rcs $@ $<

target/debug/blade-benchmarks: wasm_obj/libtea_ref.a wasm_obj/libsha256_ref.a src
	cargo build

.PHONY: build
build: target/debug/blade-benchmarks

.PHONY: run
run: target/debug/blade-benchmarks
	cargo run

.PHONY: disasm_tea
disasm_tea: wasm_obj/libtea_ref.a
	objdump -SDg $< | less
.PHONY: disasm_sha256
disasm_sha256: wasm_obj/libsha256_ref.a
	objdump -SDg $< | less

.PHONY: clean
clean:
	cargo clean
	-rm -rf wasm_obj

.PHONY: fullclean
fullclean: clean
	cd $(LUCET_REPO) && cargo clean
