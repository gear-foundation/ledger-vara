fn main() {
    println!("cargo:rerun-if-changed=script.ld");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-cfg=host_os=\"macos\"");
    }
}
