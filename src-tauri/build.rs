fn main() {
    // Tell cargo to compile the C file
    cc::Build::new()
        .file("c_src/ffi_examples.c")
        .include("c_src")
        .compile("ffi_examples");

    // Re-run build if C files change
    println!("cargo:rerun-if-changed=c_src/ffi_examples.c");
    println!("cargo:rerun-if-changed=c_src/ffi_examples.h");
}
