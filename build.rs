//! Build script for generating protobuf code from .proto files.

use std::path::Path;
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
        "asset.proto",
        "security_audit.proto"
    ];

    // Tell cargo to rerun this build script only if proto files change
    println!("cargo:rerun-if-changed={proto_root}");
    for proto_file in proto_files {
        println!("cargo:rerun-if-changed={proto_root}/{proto_file}");
    }

    // Only compile if we're not in a cached environment or if proto files changed
    let proto_out_dir = "src/api/proto";

    // Check if proto output directory exists and has files
    let should_compile = !Path::new(proto_out_dir).exists()
        || std::fs::read_dir(proto_out_dir)?.count() < proto_files.len();

    if should_compile {
        println!("cargo:warning=Compiling protobuf files...");
        tonic_build::configure()
            .out_dir(proto_out_dir)
            .build_server(true)
            .compile_protos(proto_files, &[proto_root])?;
    } else {
        println!("cargo:warning=Using cached protobuf files");
    }

    Ok(())
}
