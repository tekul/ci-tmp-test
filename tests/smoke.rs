use std::path::PathBuf;

// use rexpect::{error::Error, session::spawn_command};

#[test]
fn exo_smoke_tests() {
    let cargo_tmp_dir = env!("CARGO_TARGET_TMPDIR");
    let tmp_dir = tempfile::tempdir_in(cargo_tmp_dir).expect("Failed to create tempdir");

    assert!(tmp_dir.path().exists(), "Tmp dir not found");
    assert!(tmp_dir.path().is_dir(), "Tmp dir not a directory");

    let _project_dir = PathBuf::from(tmp_dir.path());
}
