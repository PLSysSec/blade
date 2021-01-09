# Automatically Eliminating Speculative Leaks from Cryptographic Code with Blade


This repo contains the benchmarks and analysis scripts used to test Blade in [our paper] ([video here]).

## Abstract

We introduce Blade, a new approach to automatically and efficiently eliminate speculative leaks from cryptographic code. Blade is built on the insight that to stop leaks via speculative execution, it suffices to _cut_ the dataflow from expressions that speculatively introduce secrets (_sources_) to those that leak them through the cache (_sinks_), rather than prohibit speculation altogether. We formalize this insight in a _static type system_ that (1) types each expression as either _transient_, i.e., possibly containing speculative secrets or as being _stable_, and (2) prohibits speculative leaks by requiring that all _sink_ expressions are stable. Blade relies on a new abstract primitive, `protect`, to halt speculation at fine granularity. We formalize and implement `protect` using existing architectural mechanisms, and show how Blade's type system can automatically synthesize a _minimal_ number of `protect`s to provably eliminate speculative leaks. We implement Blade in the Cranelift WebAssembly compiler and evaluate our approach by repairing several verified, yet vulnerable WebAssembly implementations of cryptographic primitives. We find that Blade can fix existing programs that leak via speculation _automatically_, without user intervention, and _efficiently_ even when using fences to implement `protect`.


## Blade benchmarks

The instructions here should allow you to reproduce Table 1 (Section 7) in that paper.

### Setup

You will need:

- This repo
- [`lucet-blade`] from https://github.com/PLSysSec/lucet-blade, on the
`blade` branch. Be sure to check out all submodules as well.
- the [WASI SDK] from https://github.com/WebAssembly/wasi-sdk
- [WABT] from https://github.com/WebAssembly/wabt (release 1.0.15 is known to work)
- [Binaryen] from https://github.com/WebAssembly/binaryen (version 90 is known to work)
- [HACL*] from https://github.com/project-everest/hacl-star/ (commit `de6a314ab` is known to work)
- Rust (e.g. from [rustup])

In the `Makefile` in this repo, adjust the variables at the top according to
the paths to each of these dependencies on your system.

Also adjust the `Cargo.toml` in this repo if necessary, with the appropriate
path to `lucet-blade` for the `lucet-runtime` dependency.

Then, `make build` in this repo, which should build our modified Lucet, all
of our Wasm examples, and the test framework.

### Running the benchmarks

In this repo, `make bench` runs all of our benchmarks. This should take
around 10-20 minutes to complete.

(You may see messages about skipped tests, which is normal, or about various
outliers, which is also normal.)

Then, use `make report` to create the table summarizing the results. This
outputs a version of the table to `stdout`, and the actual LaTeX for the
table (as presented in [our paper]) to `./analysis/table.tex`.

### Other things you can do

- After `make build`, you can inspect the generated x86 assembly for any of
our benchmarks by running `objdump -SDg` on the appropriate `.so` file in
`./wasm_obj`.
- You can also inspect the generated WebAssembly for any of the C-language
benchmarks by using `make wasm_wat/*` for the appropriate `*`.
- To compile other C code with Blade:
  - `make build` in this repo
  - Compile your code to Wasm using the WASI SDK's `clang` compiler
  (`$(WASI_SDK)/bin/clang`) and the `WASI_CLANG_FLAGS` and `WASI_LINK_FLAGS`
  from the `Makefile`
  - Then compile these Wasm files to native code using our modified Lucet
  compiler (`$(LUCET_BLADE)/target/debug/lucetc`) and the `LUCETC_FLAGS` from
  the `Makefile`.
  - You can choose the Blade mitigation using the `--blade-type` flag to
  `lucetc`:
    - For Ref: `--blade-type=none`
    - For Baseline-F: `--blade-type=baseline_fence`
    - For Baseline-S: `--blade-type=baseline_slh`
    - For Blade-F: `--blade-type=lfence`
    - For Blade-S: `--blade-type=slh`
  - You can also choose to enable the v1.1 mitigations with the
  `--blade-v1-1` flag to `lucetc` (off by default).

[our paper]: https://arxiv.org/abs/2005.00294
[`lucet-blade`]: https://github.com/PLSysSec/lucet-blade
[WASI SDK]: https://github.com/WebAssembly/wasi-sdk
[WABT]: https://github.com/WebAssembly/wabt
[Binaryen]: https://github.com/WebAssembly/binaryen
[HACL*]: https://github.com/project-everest/hacl-star/
[rustup]: https://rustup.rs
[video here]: https://app.clowdr.org/conference/popl2021/item/d96e88d9-98f9-4443-b19c-6a924f297ac9
