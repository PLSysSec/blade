# Blade benchmarks

## Initial setup

Start from [lucet-blade], which includes [wasmtime-blade] as a Git submodule.
Use the `blade` branch of lucet-blade (and wasmtime-blade).

In the `Makefile` in this repo, make sure to adjust the variables at the top
to point to your locations for `lucet-blade`, the WASI SDK, `hacl-star`, and
others.

## Doing stuff

Generally use the Makefile targets: `make build`, `make test`, `make bench`.
These will generate the appropriate `.wasm` and `.so` files and then run
the tests/benchmarks.
There are also additional useful targets `make disasm_*` and `make wasm_wat/*`
for various `*`.
Once you've run benchmarks, you can also `make report`, which prints
human-readable benchmark statistics to stdout, and creates a LaTeX table at
`analysis/table.tex`.

[lucet-blade]: https://github.com/PLSysSec/lucet-blade
[wasmtime-blade]: https://github.com/PLSysSec/wasmtime-blade
