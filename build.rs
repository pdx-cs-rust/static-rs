fn main() {
    cc::Build::new()
        .file("src/writeln_ffi_c.c")
        .compile("writeln_ffi_c");
    println!("cargo:rerun-if-changed=src/writeln_ffi_c.c");
}
