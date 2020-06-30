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

.PHONY: build
build: wasm_obj/libtea_ref.a src
	cargo build

.PHONY: run
run: build
	cargo run

.PHONY: disasm
disasm: wasm_obj/libtea_ref.a
	objdump -SDg $< | less

.PHONY: clean
clean:
	cargo clean
	-rm -rf wasm_obj

.PHONY: fullclean
fullclean: clean
	cd $(LUCET_REPO) && cargo clean
