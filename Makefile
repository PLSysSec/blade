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
	$(LUCETC) $(LUCETC_FLAGS) --blade=none $< -o $@

wasm_obj/%_lfence.so: wasm_src/%.wasm $(LUCETC)
	mkdir -p wasm_obj
	$(LUCETC) $(LUCETC_FLAGS) --blade=lfence $< -o $@

wasm_obj/%_lfence_per_block.so: wasm_src/%.wasm $(LUCETC)
	mkdir -p wasm_obj
	$(LUCETC) $(LUCETC_FLAGS) --blade=lfence_per_block $< -o $@

wasm_obj/%_slh.so: wasm_src/%.wasm $(LUCETC)
	mkdir -p wasm_obj
	$(LUCETC) $(LUCETC_FLAGS) --blade=slh $< -o $@

target/debug/blade-benchmarks: \
		wasm_obj/tea_ref.so wasm_obj/sha256_ref.so \
		wasm_obj/tea_lfence.so wasm_obj/sha256_lfence.so \
		wasm_obj/tea_lfence_per_block.so wasm_obj/sha256_lfence_per_block.so \
		wasm_obj/tea_slh.so wasm_obj/sha256_slh.so \
		src
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

disasm_tea_%: wasm_obj/tea_%.so
	objdump -SDg $< | less
disasm_sha256_%: wasm_obj/sha256_%.so
	objdump -SDg $< | less

.PHONY: clean
clean:
	cargo clean
	-rm -rf wasm_obj

.PHONY: fullclean
fullclean: clean
	cd $(LUCET_REPO) && cargo clean
