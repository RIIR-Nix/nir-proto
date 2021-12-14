fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&[
      "proposal/tvl/evaluator.proto",
      "proposal/yzix/runwork.proto",
      "proposal/yzix/store.proto",
    ], &[
      "proposal/tvl/",
      "proposal/yzix/",
    ])?;
    Ok(())
}
