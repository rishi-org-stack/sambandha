fn main() {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/gen") // you can change the generated code's location
        .compile(
            &["protos/send.proto"],
            &["."], // specify the root location to search proto dependencies
        )
        .unwrap();
}
