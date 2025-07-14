fn main() -> Result<(), Box<dyn std::error::Error>> {

    let proto_root = "./proto";
    let proto_files = &[
        "misc.proto",
        "auth.proto",
        "dashboard.proto",
        "admin_user.proto",
        "content.proto",
        "setting.proto",
        "cms.proto",
        "general.proto",
        "asset.proto"
    ];

    tonic_build::configure()
        .out_dir("src/api/proto")
        .build_server(true)
        .compile_protos(proto_files, &[proto_root])?;
    Ok(())
}
