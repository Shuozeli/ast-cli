use ast_cli::ops;
use std::path::{Path, PathBuf};

fn fixture_path(filename: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(filename)
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

// ════════════════════════════════════════════════════════════
// Simple fixture tests (regression suite)
// ════════════════════════════════════════════════════════════

#[test]
fn test_parse_line_range() {
    let path = fixture_path("rust/sample.rs");

    insta::with_settings!({snapshot_path => "snapshots/multi"}, {
        let output = ops::read::run(&path, "11:13").unwrap();
        insta::assert_snapshot!(output);
    });

    // Invalid bounds should error
    assert!(ops::read::run(&path, "15:20").is_err()); // out of bounds
    assert!(ops::read::run(&path, "10:5").is_err()); // reversed
    assert!(ops::read::run(&path, "abc").is_err()); // invalid
}

#[test]
fn test_outline_all_languages() {
    insta::with_settings!({snapshot_path => "snapshots/rust"}, {
        insta::assert_yaml_snapshot!("outline_rust", ops::outline::run(&fixture_path("rust/sample.rs")).unwrap());
    });
    insta::with_settings!({snapshot_path => "snapshots/cpp"}, {
        insta::assert_yaml_snapshot!("outline_cpp", ops::outline::run(&fixture_path("cpp/sample.cpp")).unwrap());
    });
    insta::with_settings!({snapshot_path => "snapshots/ts"}, {
        insta::assert_yaml_snapshot!("outline_ts", ops::outline::run(&fixture_path("ts/sample.ts")).unwrap());
    });
    insta::with_settings!({snapshot_path => "snapshots/python"}, {
        insta::assert_yaml_snapshot!("outline_python", ops::outline::run(&fixture_path("python/sample.py")).unwrap());
    });
    insta::with_settings!({snapshot_path => "snapshots/protobuf"}, {
        insta::assert_yaml_snapshot!("outline_proto", ops::outline::run(&fixture_path("protobuf/sample.proto")).unwrap());
    });
}

#[test]
fn test_skeleton() {
    insta::with_settings!({snapshot_path => "snapshots/python"}, {
        insta::assert_snapshot!("skeleton_python", ops::skeleton::run(&fixture_path("python/sample.py")).unwrap());
    });
    insta::with_settings!({snapshot_path => "snapshots/rust"}, {
        insta::assert_snapshot!("skeleton_rust", ops::skeleton::run(&fixture_path("rust/sample.rs")).unwrap());
    });
}

#[test]
fn test_read_by_name() {
    let rs_path = fixture_path("rust/sample.rs");
    insta::with_settings!({snapshot_path => "snapshots/rust"}, {
        insta::assert_snapshot!("read_rust_MyStruct_new", ops::read::run(&rs_path, "MyStruct::new").unwrap());
    });

    let ts_path = fixture_path("ts/sample.ts");
    insta::with_settings!({snapshot_path => "snapshots/ts"}, {
        insta::assert_snapshot!("read_ts_MyClass_doThing", ops::read::run(&ts_path, "MyClass::doThing").unwrap());
    });
}

#[test]
fn test_find() {
    let dir = fixtures_dir();
    insta::with_settings!({snapshot_path => "snapshots/multi"}, {
        let results = ops::find::run(&dir, "top_level_func", None).unwrap();
        insta::assert_yaml_snapshot!("find_all", results);

        let python_results = ops::find::run(&dir, "top_level_func", Some("function")).unwrap();
        insta::assert_yaml_snapshot!("find_python_only", python_results);
    });
}

#[test]
fn test_query() {
    let path = fixture_path("rust/sample.rs");
    insta::with_settings!({snapshot_path => "snapshots/multi"}, {
        let results = ops::query::run(&path, "(function_item name: (identifier) @name)").unwrap();
        insta::assert_yaml_snapshot!(results);
    });
}

#[test]
fn test_project() {
    let dir = fixtures_dir();
    insta::with_settings!({snapshot_path => "snapshots/multi"}, {
        let summary = ops::project::run(&dir, &[]).unwrap();
        insta::assert_yaml_snapshot!(summary);
    });
}

// ════════════════════════════════════════════════════════════
// Complex Rust tests
// ════════════════════════════════════════════════════════════

#[test]
fn test_complex_outline_rust() {
    insta::with_settings!({snapshot_path => "snapshots/rust"}, {
        let items = ops::outline::run(&fixture_path("rust/advanced.rs")).unwrap();
        insta::assert_yaml_snapshot!(items);
    });
}

#[test]
fn test_complex_skeleton_rust() {
    insta::with_settings!({snapshot_path => "snapshots/rust"}, {
        let skeleton = ops::skeleton::run(&fixture_path("rust/advanced.rs")).unwrap();
        insta::assert_snapshot!(skeleton);
    });
}

#[test]
fn test_complex_read_rust() {
    let path = fixture_path("rust/advanced.rs");
    insta::with_settings!({snapshot_path => "snapshots/rust"}, {
        insta::assert_snapshot!("insert", ops::read::run(&path, "Registry::insert").unwrap());
        insta::assert_snapshot!("inner_struct", ops::read::run(&path, "inner::InnerStruct").unwrap());
        insta::assert_snapshot!("helper", ops::read::run(&path, "inner::helper").unwrap());
    });
}

// ════════════════════════════════════════════════════════════
// Complex C++ tests
// ════════════════════════════════════════════════════════════

#[test]
fn test_complex_outline_cpp() {
    insta::with_settings!({snapshot_path => "snapshots/cpp"}, {
        let items = ops::outline::run(&fixture_path("cpp/advanced.cpp")).unwrap();
        insta::assert_yaml_snapshot!(items);
    });
}

#[test]
fn test_complex_skeleton_cpp() {
    insta::with_settings!({snapshot_path => "snapshots/cpp"}, {
        let skeleton = ops::skeleton::run(&fixture_path("cpp/advanced.cpp")).unwrap();
        insta::assert_snapshot!(skeleton);
    });
}

#[test]
fn test_complex_read_cpp() {
    let path = fixture_path("cpp/advanced.cpp");
    insta::with_settings!({snapshot_path => "snapshots/cpp"}, {
        insta::assert_snapshot!("set_position", ops::read::run(&path, "Transform::set_position").unwrap());
    });
}

// ════════════════════════════════════════════════════════════
// Complex TypeScript tests
// ════════════════════════════════════════════════════════════

#[test]
fn test_complex_outline_ts() {
    insta::with_settings!({snapshot_path => "snapshots/ts"}, {
        let items = ops::outline::run(&fixture_path("ts/advanced.ts")).unwrap();
        insta::assert_yaml_snapshot!(items);
    });
}

#[test]
fn test_complex_skeleton_ts() {
    insta::with_settings!({snapshot_path => "snapshots/ts"}, {
        let skeleton = ops::skeleton::run(&fixture_path("ts/advanced.ts")).unwrap();
        insta::assert_snapshot!(skeleton);
    });
}

#[test]
fn test_complex_read_ts() {
    let path = fixture_path("ts/advanced.ts");
    insta::with_settings!({snapshot_path => "snapshots/ts"}, {
        insta::assert_snapshot!("find_by_email", ops::read::run(&path, "UserService::findByEmail").unwrap());
        insta::assert_snapshot!("is_email", ops::read::run(&path, "Validators::isEmail").unwrap());
    });
}

// ════════════════════════════════════════════════════════════
// Complex Python tests
// ════════════════════════════════════════════════════════════

#[test]
fn test_complex_outline_python() {
    insta::with_settings!({snapshot_path => "snapshots/python"}, {
        let items = ops::outline::run(&fixture_path("python/advanced.py")).unwrap();
        insta::assert_yaml_snapshot!(items);
    });
}

#[test]
fn test_complex_skeleton_python() {
    insta::with_settings!({snapshot_path => "snapshots/python"}, {
        let skeleton = ops::skeleton::run(&fixture_path("python/advanced.py")).unwrap();
        insta::assert_snapshot!(skeleton);
    });
}

#[test]
fn test_complex_read_python() {
    let path = fixture_path("python/advanced.py");
    insta::with_settings!({snapshot_path => "snapshots/python"}, {
        insta::assert_snapshot!("normalize_email", ops::read::run(&path, "UserRepository::normalize_email").unwrap());
        insta::assert_snapshot!("permissions", ops::read::run(&path, "UserRepository::Permissions").unwrap());
    });
}

// ════════════════════════════════════════════════════════════
// Complex Protobuf tests
// ════════════════════════════════════════════════════════════

#[test]
fn test_complex_outline_proto() {
    insta::with_settings!({snapshot_path => "snapshots/protobuf"}, {
        let items = ops::outline::run(&fixture_path("protobuf/advanced.proto")).unwrap();
        insta::assert_yaml_snapshot!(items);
    });
}

// ════════════════════════════════════════════════════════════
// Cross-file search with complex fixtures
// ════════════════════════════════════════════════════════════

#[test]
fn test_complex_find_across_languages() {
    let dir = fixtures_dir();
    insta::with_settings!({snapshot_path => "snapshots/multi"}, {
        let results = ops::find::run(&dir, "validate", Some("function")).unwrap();
        insta::assert_yaml_snapshot!("validate", results);

        let method_results = ops::find::run(&dir, "method", None).unwrap();
        insta::assert_yaml_snapshot!("method", method_results);
    });
}

#[test]
fn test_complex_project_summary() {
    let dir = fixtures_dir();
    insta::with_settings!({snapshot_path => "snapshots/multi"}, {
        let summary = ops::project::run(&dir, &[]).unwrap();
        insta::assert_yaml_snapshot!(summary);
    });
}
