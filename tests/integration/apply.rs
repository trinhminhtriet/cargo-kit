use crate::utils::{init_cargo_project, CargoProject, OutputExt};

#[test]
fn apply_explicit_manifest_path() -> anyhow::Result<()> {
    let mut project = init_cargo_project()?;

    let manifest_path = "crates/inner/Cargo.toml";
    project.file(
        manifest_path,
        r#"
[package]
name = "foo"
version = "0.1.0"
edition = "2021"
"#,
    );
    project
        .cmd(&[
            "apply",
            "fast-compile",
            "dev",
            "--nightly=off",
            "--manifest-path",
            manifest_path,
        ])
        .run()?
        .assert_ok();
    insta::assert_snapshot!(project.read(manifest_path), @r###"

    [package]
    name = "foo"
    version = "0.1.0"
    edition = "2021"

    [profile.dev]
    debug = 0
    "###);

    Ok(())
}

#[test]
fn resolve_workspace_root() -> anyhow::Result<()> {
    let mut project = init_cargo_project()?;
    project.file(
        "bar/Cargo.toml",
        r#"
[package]
name = "bar"
version = "0.1.0"
edition = "2021"
"#,
    );
    project.file("bar/src/lib.rs", "");
    project.manifest(
        r#"
[workspace]
members = ["bar"]
"#,
    );

    project
        .cmd(&["apply", "fast-compile", "dev", "--nightly=off"])
        .cwd(&project.path("bar"))
        .run()?
        .assert_ok();
    insta::assert_snapshot!(project.read_manifest(), @r###"

    [workspace]
    members = ["bar"]

    [profile.dev]
    debug = 0
    "###);

    Ok(())
}

#[test]
fn apply_missing_builtin() -> anyhow::Result<()> {
    let project = init_cargo_project()?;

    apply(&project, "dev", "fast-compile")?;
    insta::assert_snapshot!(project.read_manifest(), @r###"
    [package]
    name = "foo"
    version = "0.1.0"
    edition = "2021"

    [profile.dev]
    debug = 0
    "###);

    Ok(())
}

#[test]
fn apply_existing_builtin() -> anyhow::Result<()> {
    let mut project = init_cargo_project()?;

    project.manifest(
        r#"
[package]
name = "foo"
version = "0.1.0"
edition = "2021"

[profile.dev]
debug = 1
"#,
    );

    apply(&project, "dev", "fast-compile")?;
    insta::assert_snapshot!(project.read_manifest(), @r###"

    [package]
    name = "foo"
    version = "0.1.0"
    edition = "2021"

    [profile.dev]
    debug = 0
    "###);

    Ok(())
}

#[test]
fn apply_missing_custom() -> anyhow::Result<()> {
    let project = init_cargo_project()?;

    apply(&project, "custom1", "fast-compile")?;
    insta::assert_snapshot!(project.read_manifest(), @r###"
    [package]
    name = "foo"
    version = "0.1.0"
    edition = "2021"

    [profile.custom1]
    inherits = "dev"
    opt-level = 0
    debug = 0
    strip = "none"
    lto = false
    codegen-units = 256
    incremental = true
    "###);

    Ok(())
}

#[test]
fn apply_existing_custom() -> anyhow::Result<()> {
    let mut project = init_cargo_project()?;

    project.manifest(
        r#"
[package]
name = "foo"
version = "0.1.0"
edition = "2021"

[profile.custom1]
inherits = "dev"
debug = 1
"#,
    );

    apply(&project, "custom1", "fast-compile")?;
    insta::assert_snapshot!(project.read_manifest(), @r###"

    [package]
    name = "foo"
    version = "0.1.0"
    edition = "2021"

    [profile.custom1]
    inherits = "dev"
    debug = 0
    opt-level = 0
    strip = "none"
    lto = false
    codegen-units = 256
    incremental = true
    "###);

    Ok(())
}

#[test]
fn apply_existing_keep_formatting() -> anyhow::Result<()> {
    let mut project = init_cargo_project()?;

    project.manifest(
        r#"
[package]
name = "foo"
version = "0.1.0"
edition = "2021"

[profile.dev]

lto =      "thin"

debug = 1   # Foo

codegen-units    = 10
"#,
    );

    apply(&project, "dev", "fast-compile")?;
    insta::assert_snapshot!(project.read_manifest(), @r###"

    [package]
    name = "foo"
    version = "0.1.0"
    edition = "2021"

    [profile.dev]

    lto =      false

    debug = 0   # Foo

    codegen-units    = 256
    "###);

    Ok(())
}

#[test]
fn apply_fast_runtime_template() -> anyhow::Result<()> {
    let project = init_cargo_project()?;

    apply(&project, "custom", "fast-runtime")?;
    insta::assert_snapshot!(project.read_manifest(), @r###"
    [package]
    name = "foo"
    version = "0.1.0"
    edition = "2021"

    [profile.custom]
    inherits = "release"
    opt-level = 3
    debug = false
    strip = "none"
    lto = true
    codegen-units = 1
    incremental = false
    panic = "abort"
    "###);

    insta::assert_snapshot!(project.read_config(), @r###"
    [build]
    rustflags = ["-Ctarget-cpu=native"]
    "###);

    Ok(())
}

#[test]
fn apply_min_size_template() -> anyhow::Result<()> {
    let project = init_cargo_project()?;

    apply(&project, "custom", "min-size")?;
    insta::assert_snapshot!(project.read_manifest(), @r###"
    [package]
    name = "foo"
    version = "0.1.0"
    edition = "2021"

    [profile.custom]
    inherits = "release"
    opt-level = "z"
    debug = false
    strip = true
    lto = true
    codegen-units = 1
    incremental = false
    panic = "abort"
    "###);

    Ok(())
}

fn apply(project: &CargoProject, profile: &str, template: &str) -> anyhow::Result<()> {
    project
        .cmd(&["apply", template, profile, "--nightly=off"])
        .run()?
        .assert_ok();
    Ok(())
}
