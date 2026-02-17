fn main() -> Result<(), Box<dyn std::error::Error>> {

    let proto_root = "./proto";
    let proto_files = &[
        "helloworld.proto",
        "misc.proto",
        // "auth.proto",
        // "dashboard.proto",
        // "admin_user.proto",
        // "content.proto",
        // "setting.proto",
        // "cms.proto",
        // "general.proto",
        // "asset.proto",
        // "security_audit.proto",
    ];

    let proto_out_dir = "src/infra/grpc";

    tonic_prost_build::configure()
            .out_dir(proto_out_dir)
            .build_server(true)
            .compile_protos(proto_files, &[proto_root])?;


    // tonic_prost_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}
