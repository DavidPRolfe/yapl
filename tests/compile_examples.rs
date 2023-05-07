use std::path::{Path, PathBuf};
use yapl::{compile, CompilerError};

/// Simple tests for all examples in lang examples to make sure that everything compiles

fn load_example(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("lang_examples").join(name)
}

#[test]
fn expressions() -> Result<(), CompilerError> {
    compile(load_example("expressions.ypl").to_str().unwrap())
}

#[test]
fn factorial() -> Result<(), CompilerError> {
    compile(load_example("factorial.ypl").to_str().unwrap())
}

// TODO: Delete this when covered by integration test
#[test]
fn scratch_pad() -> Result<(), CompilerError> {
    compile(load_example("scratch_pad.ypl").to_str().unwrap())
}