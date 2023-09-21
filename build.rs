fn main() {
    println!("cargo:rerun-if-changed=script.ld");

    if cfg!(target_os = "macos") {
        let xcrun = std::process::Command::new("xcrun")
            .arg("--show-sdk-path")
            .output()
            .expect("Failed to execute `xcrun` command");

        let sdk_path = String::from_utf8_lossy(&xcrun.stdout);

        println!("cargo:rustc-env=CPATH={sdk_path}//usr/include");
        println!("cargo:rustc-cfg=host_os=\"macos\"");
    }
}
