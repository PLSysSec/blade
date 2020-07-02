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
	$(LUCETC) $(LUCETC_FLAGS) --blade-type=none $< -o $@

wasm_obj/%_lfence_with_v1_1.so: wasm_src/%.wasm $(LUCETC)
	mkdir -p wasm_obj
	$(LUCETC) $(LUCETC_FLAGS) --blade-type=lfence --blade-v1-1 $< -o $@

wasm_obj/%_lfence_no_v1_1.so: wasm_src/%.wasm $(LUCETC)
	mkdir -p wasm_obj
	$(LUCETC) $(LUCETC_FLAGS) --blade-type=lfence $< -o $@

wasm_obj/%_lfence_per_block_with_v1_1.so: wasm_src/%.wasm $(LUCETC)
	mkdir -p wasm_obj
	$(LUCETC) $(LUCETC_FLAGS) --blade-type=lfence_per_block --blade-v1-1 $< -o $@

wasm_obj/%_lfence_per_block_no_v1_1.so: wasm_src/%.wasm $(LUCETC)
	mkdir -p wasm_obj
	$(LUCETC) $(LUCETC_FLAGS) --blade-type=lfence_per_block $< -o $@

wasm_obj/%_slh_with_v1_1.so: wasm_src/%.wasm $(LUCETC)
	mkdir -p wasm_obj
	$(LUCETC) $(LUCETC_FLAGS) --blade-type=slh --blade-v1-1 $< -o $@

wasm_obj/%_slh_no_v1_1.so: wasm_src/%.wasm $(LUCETC)
	mkdir -p wasm_obj
	$(LUCETC) $(LUCETC_FLAGS) --blade-type=slh $< -o $@

target/debug/blade-benchmarks: \
		wasm_obj/tea_ref.so wasm_obj/sha256_ref.so \
		wasm_obj/tea_lfence_with_v1_1.so wasm_obj/sha256_lfence_with_v1_1.so \
		wasm_obj/tea_lfence_no_v1_1.so wasm_obj/sha256_lfence_no_v1_1.so \
		wasm_obj/tea_lfence_per_block_with_v1_1.so wasm_obj/sha256_lfence_per_block_with_v1_1.so \
		wasm_obj/tea_lfence_per_block_no_v1_1.so wasm_obj/sha256_lfence_per_block_no_v1_1.so \
		wasm_obj/tea_slh_with_v1_1.so wasm_obj/sha256_slh_with_v1_1.so \
		wasm_obj/tea_slh_no_v1_1.so wasm_obj/sha256_slh_no_v1_1.so \
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
