fn main() {
    let mut config = prost_build::Config::new();
    config.btree_map(&["."]);

    tonic_build::configure()
        .type_attribute(
            "codectrl.data.backtrace_data.BacktraceData",
            r#"#[derive(Serialize, Deserialize)]"#,
        )
        .type_attribute(
            "codectrl.data.log.Log",
            r#"#[derive(Serialize, Deserialize)]"#,
        )
        .type_attribute(
            "codectrl.logs_service.Connection",
            r#"#[derive(Serialize, Deserialize)]"#,
        )
        .type_attribute(
            "codectrl.logs_service.ServerDetails",
            r#"#[derive(Serialize, Deserialize)]"#,
        )
        .build_server(false)
        .compile_with_config(
            config,
            &[
                "../proto/cc_service.proto",
                "../proto/backtrace_data.proto",
                "../proto/log.proto",
            ],
            &["../proto"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {e:#?}"));
}
