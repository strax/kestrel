use std::env;
use std::path::PathBuf;

fn main() {
    cmake::Config::new("binaryen")
        .define("BUILD_TESTS", "OFF")
        .define("BUILD_TOOLS", "OFF")
        .define("BUILD_STATIC_LIB", "ON")
        .build();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search=native={}", out_dir.join("lib").to_str().unwrap());
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=static=binaryen");

    bindgen::builder()
        .clang_args(["-I", out_dir.join("include").to_str().unwrap()])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .layout_tests(false)
        .header("wrapper.h")
        .allowlist_function("Binaryen.*")
        .allowlist_function("Relooper.*")
        .allowlist_function("ExpressionRunner.*")
        .allowlist_type("Binaryen.*")
        .allowlist_type("Relooper.*")
        .allowlist_type("ExpressionRunner.*")
        .allowlist_type("CExpressionRunner")
        .blocklist_type("size_t")
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindings.rs"))
        .unwrap();
}