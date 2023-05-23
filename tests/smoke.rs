use std::path::PathBuf;

// use rexpect::{error::Error, session::spawn_command};

#[test]
fn exo_smoke_tests() {
    let cargo_tmp_dir = env!("CARGO_TARGET_TMPDIR");
    let tmp_dir = tempfile::tempdir_in(cargo_tmp_dir).expect("Failed to create tempdir");

    assert!(tmp_dir.path().exists(), "Tmp dir not found");
    assert!(tmp_dir.path().is_dir(), "Tmp dir not a directory");

    let mut project_dir = PathBuf::from(tmp_dir.path());
    project_dir.push("mariposas");
    std::fs::create_dir(project_dir.clone()).expect("Failed to create project_dir");
    assert!(project_dir.exists(), "Project dir {project_dir:?} not found");
    assert!(project_dir.is_dir(), "Project dir {project_dir:?} not a directory");
}
