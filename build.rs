fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(&["proto/helloworld/helloworld.proto"], &["proto"])
        .expect("Could not compile proto files");
}
