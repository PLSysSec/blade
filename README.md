# Blade benchmarks

## Initial setup

Start from [lucet-spectre]; check out tag `SWIVEL_BASE_POINT`.

In the `wasmtime` directory of `lucet-spectre`, add
https://github.com/PLSysSec/wasmtime-blade as a remote and check
out the `blade` branch from that remote.

Also you may need to comment out the version checking from the Lucet module
loading code in `lucet-spectre/lucet-runtime/lucet-runtime-internals/src/module/dl.rs`.

## Doing stuff

Generally use the Makefile targets: `make build`, `make test`, `make bench`.

[lucet-spectre]: https://github.com/PLSysSec/lucet-spectre
