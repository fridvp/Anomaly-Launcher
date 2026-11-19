fn main() {
    let mut res = winres::WindowsResource::new();
    res.set("ProductName", "Anomaly Launcher");
    res.set("FileDescription", "Anomaly Launcher");
    res.set("LegalCopyright", "Copyright (C) 2024");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let rc_path = std::path::Path::new(&manifest_dir).join("launcher.rc");
    res.set_resource_file(rc_path.to_str().unwrap());

    res.set_windres_path("x86_64-w64-mingw32-windres");
    res.compile()
    .expect("Failed to run the Windows resource compiler (rc.exe)");
}
