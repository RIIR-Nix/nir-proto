fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&[
      "proposal/tvl/evaluator.proto",
      "proposal/yzix/hashty.proto",
      "proposal/yzix/done.proto",
      "proposal/yzix/tasks.proto",
    ], &[
      "proposal/tvl/",
      "proposal/yzix/",
    ])?;
    Ok(())
}
