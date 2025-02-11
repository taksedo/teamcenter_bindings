use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    println!("=========================>  START");
    let out_dir_string = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(out_dir_string);

    bindgen::builder()
        .header("wrapper_manual.h")
        .clang_arg("-v")
        .clang_arg("--include-directory=include")
        .clang_arg("-DIPLIB=none")
        .clang_macro_fallback()
        .derive_debug(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let lib_dir = Path::new("D:/code/rust/tc-binding-experiments/tc-bindings/tc-bindgen-sys/lib");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    // println!("cargo:rustc-link-arg=RpcRT4.Lib");
    // println!("cargo:rustc-link-arg=ws2_32.Lib");
    // println!("cargo:rustc-link-arg=wsock32.Lib");
    // println!("cargo:rustc-link-arg=advapi32.Lib");
    // println!("cargo:rustc-link-arg=msvcrt.Lib");
    // println!("cargo:rustc-link-arg=msvcrtd.Lib");
    // println!("cargo:rustc-link-arg=oldnames.Lib");
    // println!("cargo:rustc-link-arg=kernel32.Lib");
    // println!("cargo:rustc-link-arg=winmm.Lib");
    // println!("cargo:rustc-link-arg=/DEBUG");
    // println!("cargo:rustc-link-arg=/NOLOGO");
    // println!("cargo:rustc-link-arg=/SUBSYSTEM:CONSOLE");
    // println!("cargo:rustc-link-arg=/NODEFAULTLIB:libcmt");

    // Считываем все файлы в директории
    if let Ok(entries) = fs::read_dir(lib_dir) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".lib") && file_name.starts_with("libtc") {
                    // if file_name.ends_with(".lib") { //is not working for some reason
                    let lib_name = file_name.trim_end_matches(".lib");
                    println!("cargo:rustc-link-lib=static={}", lib_name);
                }
            }
        }
    } else {
        panic!("Failed to read library directory");
    }

    println!("cargo:rustc-link-lib=static={}", "customize_am");
    println!("cargo:rustc-link-lib=static={}", "libpom");
    println!("cargo:rustc-link-lib=static={}", "libbase_utils");
    println!("cargo:rustc-link-arg=itk_main.obj");
}
