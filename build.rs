use std::env;
use std::path::PathBuf;

fn main() {
    let obj_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}/wasm_obj/", obj_dir);
    println!("cargo:rustc-link-lib=tea_ref");
    println!("cargo:rustc-link-lib=sha256_ref");
    println!("cargo:rerun-if-changed=wasm_src/tea.h");
    println!("cargo:rerun-if-changed=wasm_src/sha256.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let tea_bindings = bindgen::Builder::default()
        .header("wasm_src/tea.h")
        .whitelist_function("guest_func_encrypt")
        .whitelist_function("guest_func_decrypt")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings for tea.h");

    tea_bindings
        .write_to_file(out_path.join("tea_bindings.rs"))
        .expect("Failed to write bindings for tea.h");

    let sha256_bindings = bindgen::Builder::default()
        .header("wasm_src/sha256.h")
        .whitelist_function("guest_func_init")
        .whitelist_function("guest_func_update")
        .whitelist_function("guest_func_final")
        .whitelist_type("SHA256_CTX")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings for sha256.h");

    sha256_bindings
        .write_to_file(out_path.join("sha256_bindings.rs"))
        .expect("Failed to write bindings for sha256.h");
}
