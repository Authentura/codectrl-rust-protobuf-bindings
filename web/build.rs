fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &[
                "../proto/cc_service.proto",
                "../proto/backtrace_data.proto",
                "../proto/log.proto",
            ],
            &["../proto"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {e:#?}"));
}
