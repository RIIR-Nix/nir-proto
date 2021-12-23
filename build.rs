fn main() -> std::io::Result<()> {
    prost_build::compile_protos(
        &[
            "proposal/tvix/evaluator.proto",
            "proposal/yzix/hashty.proto",
            "proposal/yzix/done.proto",
            "proposal/yzix/tasks.proto",
        ],
        &["proposal/tvix/", "proposal/yzix/"],
    )?;
    Ok(())
}
