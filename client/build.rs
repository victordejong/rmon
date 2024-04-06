fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/livemetrics.proto")?;
    tonic_build::compile_protos("proto/hostfacts.proto")?;
    Ok(())
}
