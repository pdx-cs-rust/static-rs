fn main() {
    let name = "writeln_ffi_c";
    let cname = format!("src/{}.c", name);
    cc::Build::new()
        .file(&cname)
        .compile(name);
    println!("cargo:rerun-if-changed={}", cname);
}
