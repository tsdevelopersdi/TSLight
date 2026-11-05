//! Windows only: embed version info (VERSIONINFO) into `tslight-cli.exe`.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LIGHTCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set("ProductName", "TSLight")
        .set("CompanyName", "Solusi Daya Indonesia")
        .set("FileDescription", "TSLight command-line tool")
        .set("LegalCopyright", "Copyright (c) Solusi Daya Indonesia. MIT OR Apache-2.0.")
        .set("OriginalFilename", "tslight-cli.exe")
        .set("InternalName", "tslight-cli");
    if let Err(e) = res.compile() {
        if std::env::var_os("LIGHTCRAFT_REQUIRE_WINRES").is_some() {
            panic!("embedding Windows resources failed: {e}");
        }
        println!("cargo:warning=tslight-cli.exe built without version resources: {e}");
    }
}
