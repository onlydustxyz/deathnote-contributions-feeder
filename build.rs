fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(
            &["proto/apibara/application/indexer_service.proto"],
            &["proto", "proto/googleapis"],
        )?;
    Ok(())
}
