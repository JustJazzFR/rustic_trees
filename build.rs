use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("src/c/cbasictrees.c")
        .compile("cbasictrees");
    println!("cargo:rerun-if-changed=src/c/cbasictrees.h");
    println!("cargo:rerun-if-changed=src/c/cbasictrees.c");

    //BINDGEN

    let bindings = bindgen::Builder::default()
        .header("src/c/cbasictrees.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindigns");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings.");
    println!("cargo:rerun-if-changed=src/c/cbasictrees.h");
    println!("cargo:rerun-if-changed=src/c/cbasictrees.c");
}
