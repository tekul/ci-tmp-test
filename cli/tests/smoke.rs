#![cfg(unix)]
use std::{
    path::{Path, PathBuf},
    process::Command,
};

use rexpect::{error::Error, session::spawn_command};

fn exe<I>(cwd: impl AsRef<Path>, args: I) -> Command
where
    I: IntoIterator<Item = &'static str>,
{
    let bin = env!("CARGO_BIN_EXE_ci-tmp-test");
    dbg!("bin");

    let mut cmd = Command::new(bin);
    cmd.current_dir(cwd).args(args);
    cmd
}

#[test]
fn exo_smoke_tests() -> Result<(), Error> {
    let cargo_tmp_dir = env!("CARGO_TARGET_TMPDIR");
    let tmp_dir = tempfile::tempdir_in(cargo_tmp_dir).expect("Failed to create tempdir");

    assert!(tmp_dir.path().exists(), "Tmp dir not found");
    assert!(tmp_dir.path().is_dir(), "Tmp dir not a directory");

    let cmd = exe(tmp_dir.path(), ["mariposas"]);
    let mut p = spawn_command(cmd, Some(5000))?;
    dbg!(p.exp_eof()?);

    let mut project_dir = PathBuf::from(tmp_dir.path());
    project_dir.push("mariposas");
    // std::fs::create_dir(project_dir.clone()).expect("Failed to create project_dir");
    assert!(
        project_dir.exists(),
        "Project dir {project_dir:?} not found"
    );
    assert!(
        project_dir.is_dir(),
        "Project dir {project_dir:?} not a directory"
    );
    Ok(())
}
