fn main() -> Result<(), Box<dyn std::error::Error>> {

    let proto_root = "./proto";
    let proto_files = &[
        "misc.proto",
        "auth.proto",
        "dashboard.proto",
    ];

    tonic_build::configure()
        .build_server(true)
        .compile_protos(proto_files, &[proto_root])?;
    Ok(())
}