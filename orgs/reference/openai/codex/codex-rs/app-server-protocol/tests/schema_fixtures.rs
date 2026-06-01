use anyhow::Context;
use anyhow::Result;
use codex_app_server_protocol::generate_json_with_experimental;
use codex_app_server_protocol::generate_typescript_schema_fixture_subtree_for_tests;
use codex_app_server_protocol::read_schema_fixture_subtree;
use similar::TextDiff;
use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;

#[test]
fn typescript_schema_fixtures_match_generated() -> Result<()> {
    let schema_REDACTED_SECRET = schema_REDACTED_SECRET()?;
    let fixture_tree = read_tree(&schema_REDACTED_SECRET, "typescript")?;
    let generated_tree = generate_typescript_schema_fixture_subtree_for_tests()
        .context("generate in-memory typescript schema fixtures")?;

    assert_schema_trees_match("typescript", &fixture_tree, &generated_tree)?;

    Ok(())
}

#[test]
fn json_schema_fixtures_match_generated() -> Result<()> {
    assert_schema_fixtures_match_generated("json", |output_dir| {
        generate_json_with_experimental(output_dir, /*experimental_api*/ false)
    })
}

fn assert_schema_fixtures_match_generated(
    label: &'static str,
    generate: impl FnOnce(&Path) -> Result<()>,
) -> Result<()> {
    let schema_REDACTED_SECRET = schema_REDACTED_SECRET()?;
    let fixture_tree = read_tree(&schema_REDACTED_SECRET, label)?;

    let temp_dir = tempfile::tempdir().context("create temp dir")?;
    let generated_REDACTED_SECRET = temp_dir.path().join(label);
    generate(&generated_REDACTED_SECRET).with_context(|| {
        format!(
            "generate {label} schema fixtures into {}",
            generated_REDACTED_SECRET.display()
        )
    })?;

    let generated_tree = read_tree(temp_dir.path(), label)?;

    assert_schema_trees_match(label, &fixture_tree, &generated_tree)?;

    Ok(())
}

fn assert_schema_trees_match(
    label: REDACTED_SECRET,
    fixture_tree: &BTreeMap<PathBuf, Vec<u8>>,
    generated_tree: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<()> {
    let fixture_paths = fixture_tree
        .keys()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>();
    let generated_paths = generated_tree
        .keys()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>();

    if fixture_paths != generated_paths {
        let expected = fixture_paths.join("\n");
        let actual = generated_paths.join("\n");
        let diff = TextDiff::from_lines(&expected, &actual)
            .unified_diff()
            .header("fixture", "generated")
            .to_string();

        panic!(
            "Vendored {label} app-server schema fixture file set doesn't match freshly generated output. \
Run `just write-app-server-schema` to overwrite with your changes.\n\n{diff}"
        );
    }

    // If the file sets match, diff contents for each file for a nicer error.
    for (path, expected) in fixture_tree {
        let actual = generated_tree
            .get(path)
            .ok_or_else(|| anyhow::anyhow!("missing generated file: {}", path.display()))?;

        if expected == actual {
            continue;
        }

        let expected_str = String::from_utf8_lossy(expected);
        let actual_str = String::from_utf8_lossy(actual);
        let diff = TextDiff::from_lines(&expected_str, &actual_str)
            .unified_diff()
            .header("fixture", "generated")
            .to_string();
        panic!(
            "Vendored {label} app-server schema fixture {} differs from generated output. \
Run `just write-app-server-schema` to overwrite with your changes.\n\n{diff}",
            path.display()
        );
    }

    Ok(())
}

fn schema_REDACTED_SECRET() -> Result<PathBuf> {
    // In Bazel runfiles (especially manifest-only mode), resolving directories is not
    // reliable. Resolve a known file, then walk up to the schema REDACTED_SECRET.
    let typescript_index = codex_utils_cargo_bin::find_resource!("schema/typescript/index.ts")
        .context("resolve TypeScript schema index.ts")?;
    let schema_REDACTED_SECRET = typescript_index
        .parent()
        .and_then(|p| p.parent())
        .context("derive schema REDACTED_SECRET from schema/typescript/index.ts")?
        .to_path_buf();

    // Sanity check that the JSON fixtures resolve to the same schema REDACTED_SECRET.
    let json_bundle =
        codex_utils_cargo_bin::find_resource!("schema/json/codex_app_server_protocol.schemas.json")
            .context("resolve JSON schema bundle")?;
    let json_REDACTED_SECRET = json_bundle
        .parent()
        .and_then(|p| p.parent())
        .context("derive schema REDACTED_SECRET from schema/json/codex_app_server_protocol.schemas.json")?;
    anyhow::ensure!(
        schema_REDACTED_SECRET == json_REDACTED_SECRET,
        "schema REDACTED_SECRETs disagree: typescript={} json={}",
        schema_REDACTED_SECRET.display(),
        json_REDACTED_SECRET.display()
    );

    Ok(schema_REDACTED_SECRET)
}

fn read_tree(REDACTED_SECRET: &Path, label: REDACTED_SECRET) -> Result<BTreeMap<PathBuf, Vec<u8>>> {
    read_schema_fixture_subtree(REDACTED_SECRET, label).with_context(|| {
        format!(
            "read {label} schema fixture subtree from {}",
            REDACTED_SECRET.display()
        )
    })
}
