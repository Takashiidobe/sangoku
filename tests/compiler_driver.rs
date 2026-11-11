use sangoku_test_support::{Result, ensure, harness, tempfile};
use std::{fs, process::Command};

fn run(command: &mut Command) -> Result<String> {
    let output = command.output()?;
    ensure!(
        output.status.success(),
        "{command:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(String::from_utf8(output.stdout)?)
}

#[test]
fn linked_backend_supports_target_codegen_and_cargo_host_tools() -> Result<()> {
    let root = harness::root();
    run(Command::new("cargo").current_dir(&root).args([
        "+gbc-lr35902",
        "build",
        "--release",
        "--locked",
        "--manifest-path",
        "third_party/rustc_codegen_cranelift/Cargo.toml",
    ]))?;
    let directory = tempfile::tempdir()?;
    let driver = directory.path().join("sangoku rustc");
    fs::copy(
        root.join("third_party/rustc_codegen_cranelift/target/release/sangoku-rustc"),
        &driver,
    )?;
    let dynamic = run(Command::new("readelf").arg("-d").arg(&driver))?;
    ensure!(
        !dynamic.contains("librustc_codegen_cranelift"),
        "driver links the backend dynamically: {dynamic}"
    );
    let cfg = run(Command::new(&driver)
        .env_remove("LD_LIBRARY_PATH")
        .args(["-Zunstable-options", "--target"])
        .arg(root.join("targets/lr35902.json"))
        .args(["--print", "cfg"]))?;
    ensure!(cfg.contains("target_arch=\"z80\""), "wrong target: {cfg}");
    let assembly = directory.path().join("answer.asm");
    run(Command::new(&driver)
        .env_remove("LD_LIBRARY_PATH")
        .args(["-Zunstable-options", "--target"])
        .arg(root.join("targets/lr35902.json"))
        .args(["--crate-type=lib", "--emit=asm", "-Cpanic=immediate-abort"])
        .arg(root.join("tests/fixtures/driver/answer.rs"))
        .arg("-o")
        .arg(&assembly))?;
    let assembly = fs::read_to_string(assembly)?;
    ensure!(
        assembly.contains("SECTION") && assembly.contains("__rust_answer"),
        "expected RGBDS assembly: {assembly}"
    );

    fs::write(
        directory.path().join("Cargo.toml"),
        "[package]\nname = \"driver-probe\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\
         [lib]\npath = \"lib.rs\"\n\
         [dependencies]\nprobe-macros = { path = \"macros\" }\n[workspace]\n",
    )?;
    let source = fs::read_to_string(root.join("tests/fixtures/driver/answer.rs"))?.replace(
        "#[unsafe(no_mangle)]",
        "#[probe_macros::identity]\n#[unsafe(no_mangle)]",
    );
    fs::write(
        directory.path().join("lib.rs"),
        format!("{source}\n#[cfg(not(build_script_ran))]\nuse missing_build_script;\n"),
    )?;
    fs::write(
        directory.path().join("build.rs"),
        "fn main() {\n\
         println!(\"cargo:rustc-check-cfg=cfg(build_script_ran)\");\n\
         println!(\"cargo:rustc-cfg=build_script_ran\");\n}\n",
    )?;
    fs::create_dir(directory.path().join("macros"))?;
    fs::write(
        directory.path().join("macros/Cargo.toml"),
        "[package]\nname = \"probe-macros\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\
         [lib]\nproc-macro = true\npath = \"lib.rs\"\n",
    )?;
    fs::write(
        directory.path().join("macros/lib.rs"),
        "use proc_macro::TokenStream;\n\
         #[proc_macro_attribute]\n\
         pub fn identity(_: TokenStream, item: TokenStream) -> TokenStream { item }\n",
    )?;
    run(Command::new("cargo")
        .current_dir(directory.path())
        .env("RUSTC", &driver)
        .env_remove("RUSTC_WRAPPER")
        .env_remove("RUSTC_WORKSPACE_WRAPPER")
        .env_remove("RUSTFLAGS")
        .env_remove("LD_LIBRARY_PATH")
        .env("CARGO_ENCODED_RUSTFLAGS", "-Zunstable-options")
        .args([
            "+gbc-lr35902",
            "check",
            "-Zjson-target-spec",
            "--offline",
            "--target",
        ])
        .arg(root.join("targets/lr35902.json"))
        .arg("--target-dir")
        .arg(directory.path().join("target")))?;
    Ok(())
}
