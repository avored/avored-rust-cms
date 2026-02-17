//! Build script for generating protobuf code from .proto files.


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = "../proto";
    let proto_files = &[
        "misc.proto"
    ];
    let proto_out_dir = "src/proto";

    if true {
        println!("cargo:warning=Compiling protobuf files...");
        tonic_prost_build::configure()
            .out_dir(proto_out_dir)
            .build_server(false)
            .build_transport(false)
            .compile_protos(proto_files, &[proto_root])?;
    } else {
        println!("cargo:warning=Using cached protobuf files");
    }

    Ok(())
}
