use std::env;
use std::path::PathBuf;

fn main() {
    let obj_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}/wasm_obj/", obj_dir);
    println!("cargo:rustc-link-lib=tea_ref");
    println!("cargo:rerun-if-changed=wasm_src/tea.h");

    let bindings = bindgen::Builder::default()
        .header("wasm_src/tea.h")
        .whitelist_function("guest_func_encrypt")
        .whitelist_function("guest_func_decrypt")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings for tea.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings for tea.h");
}
