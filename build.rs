fn main() {
    let out_dir = format!("{}/protos", std::env::var("OUT_DIR").unwrap());

    std::fs::create_dir_all(&out_dir).unwrap();

    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(out_dir)
        .inputs(["protos/rendezvous.proto", "protos/message.proto"])
        .include("protos")
        .customize(protobuf_codegen::Customize::default().tokio_bytes(true))
        .run()
        .expect("Codegen failed.");

    let role = std::env::var("INODESK_ROLE").unwrap_or_default();
    let (app_name, bundle_org, win_exe, uri_scheme) = match role.as_str() {
        "user" => (
            "IndoDesk User",
            "com.indoteknisi.indodesk.user",
            "indodesk-user",
            "indodesk",
        ),
        "teknisi" => (
            "IndoDesk Teknisi",
            "com.indoteknisi.indodesk.teknisi",
            "indodesk-teknisi",
            "indodesk",
        ),
        _ => ("IndoDesk", "com.carriez", "indodesk", "indodesk"),
    };
    println!("cargo:rustc-env=INODESK_APP_NAME={app_name}");
    println!("cargo:rustc-env=INODESK_BUNDLE_ORG={bundle_org}");
    println!("cargo:rustc-env=INODESK_WIN_EXE={win_exe}");
    println!("cargo:rustc-env=INODESK_URI_SCHEME={uri_scheme}");
    println!("cargo:rerun-if-env-changed=INODESK_ROLE");
}
