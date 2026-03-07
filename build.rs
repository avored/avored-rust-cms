fn main() -> Result<(), Box<dyn std::error::Error>> {

    let proto_root = "./proto";
    let proto_files = &[
        "admin_user_message.proto",
        "helloworld.proto",
        "misc.proto",
        "auth_user.proto",
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
            .type_attribute(".", "#[derive(serde::Serialize)]")
            .extern_path(".google.protobuf.Timestamp", "::pbjson_types::Timestamp")
            .out_dir(proto_out_dir)
            .build_server(true)
            .compile_protos(proto_files, &[proto_root])?;


    // tonic_prost_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}
