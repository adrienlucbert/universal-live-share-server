fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &["proto/live-share/live-share.proto"],
            &["proto/live-share"],
        )
        .expect("Could not compile proto files");
}
