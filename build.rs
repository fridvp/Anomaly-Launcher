
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set("ProductName", "Anomaly Launcher");
    res.set("FileDescription", "Anomaly Launcher");
    res.set("LegalCopyright", "Copyright (C) 2024");
    res.set_resource_file("launcher.rc");
    res.compile()
        .expect("Failed to run the Windows resource compiler (rc.exe)");
}
