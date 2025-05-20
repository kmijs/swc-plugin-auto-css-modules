use auto_css_modules::{auto_css_modules, Config};
use std::{fs::read_to_string, path::PathBuf};
use swc_core::ecma::{
    parser::{EsSyntax, Syntax},
    transforms::testing::test_fixture,
};
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    })
}

#[fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let dir = input.parent().unwrap();
    let config = read_to_string(dir.join("config.json")).expect("failed to read config.json");
    println!("---- Config -----\n{}", config);
    let config: Config = serde_json::from_str(&config).unwrap();
    test_fixture(
        syntax(),
        &|_t| auto_css_modules(config.clone()),
        &input,
        &dir.join("output.js"),
        Default::default(),
    );
}
