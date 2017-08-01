#![cfg(feature = "compiletest_rs")]
extern crate compiletest_rs as compiletest;

fn run_mode(mode: &'static str) {
    let mut config = compiletest::Config::default();

    config.mode = mode.parse().expect("Invalid mode");
    config.src_base = format!("tests/{}", mode).into();
    config.target_rustcflags = Some("-L target/debug/deps/".to_owned());

    compiletest::run_tests(&config);
}

#[test]
fn compile_fail() {
    run_mode("compile-fail");
}
