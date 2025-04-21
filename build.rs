// fn main() -> Result<(), Box<dyn std::error::Error>> {
//
//     let proto_root = "./proto";
//     let proto_files = &[
//         "misc.proto",
//         "auth.proto",
//         "dashboard.proto",
//         "admin_user.proto",
//     ];
//
//     tonic_build::configure()
//         .build_server(true)
//         .compile_protos(proto_files, &[proto_root])?;
//     Ok(())
// }


fn main() {
    tonic_build::configure()
        .compile_protos(&["proto/echo.proto"], &["proto"])
        .unwrap();
}