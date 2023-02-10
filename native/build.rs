use std::path::Path;

fn main() {
    let mut config = prost_build::Config::new();
    config.btree_map(&["."]);

    let path = Path::new("../proto");

    if !path.exists() {
        panic!(
            "Protos directory does not exist at \"../proto\", please sync + update the submodule."
        )
    }

    if let Ok(contents) = path.read_dir() {
        if contents
            .filter_map(|item| item.ok())
            .filter_map(|item| item.file_name().into_string().ok())
            .filter(|file_name| file_name.ends_with(".proto"))
            .count()
            == 0
        {
            panic!("Protos directory does exist at \"../proto\" but does not contain protobuf files. Please sync + update the submodule.");
        }
    }

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
        .type_attribute(
            "codectrl.auth_service.TokenIntent",
            r#"#[derive(Serialize, Deserialize)]"#,
        )
        .type_attribute(
            "codectrl.auth_service.TokenPermissions",
            r#"#[derive(Serialize, Deserialize)]"#,
        )
        .type_attribute(
            "codectrl.auth_service.Token",
            r#"#[derive(Serialize, Deserialize)]"#,
        )
        .type_attribute(
            "codectrl.auth_service.Name",
            r#"#[derive(Serialize, Deserialize)]"#,
        )
        .type_attribute("codectrl.auth_service.Name", r#"#[repr(transparent)]"#)
        .build_server(!cfg!(target_arch = "wasm32"))
        .compile_with_config(
            config,
            &[
                "../proto/cc_service.proto",
                "../proto/backtrace_data.proto",
                "../proto/log.proto",
                "../proto/auth.proto",
            ],
            &["../proto"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {e:#?}"));
}
