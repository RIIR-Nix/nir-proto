fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["src/runwork.proto", "src/store.proto"], &["src/"])?;
    Ok(())
}
