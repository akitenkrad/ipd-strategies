use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const BINARY: &str = env!("CARGO_BIN_EXE_ipd-strategies");
const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

struct TempDir(PathBuf);

impl TempDir {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time must be after the Unix epoch")
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("ipd-strategies-{}-{nonce}", std::process::id()));
        fs::create_dir(&path).expect("temporary directory should be created");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn fixture(name: &str) -> Vec<u8> {
    fs::read(Path::new(MANIFEST_DIR).join("tests/fixtures").join(name))
        .expect("fixture should be readable")
}

#[test]
fn legacy_stdout_matches_fixture() {
    let output = Command::new(BINARY)
        .arg("legacy")
        .output()
        .expect("legacy command should run");

    assert!(output.status.success(), "legacy command failed: {output:?}");
    assert_eq!(output.stdout, fixture("legacy_stdout.txt"));
}

#[test]
fn tournament_writes_expected_output() {
    let temp_dir = TempDir::new();
    let output = Command::new(BINARY)
        .current_dir(temp_dir.path())
        .output()
        .expect("tournament command should run");

    assert!(
        output.status.success(),
        "tournament command failed: {output:?}"
    );
    let ranking = fixture("ranking.csv");
    assert_eq!(output.stdout, ranking);

    let out_dir = temp_dir.path().join("out");
    let mut files: Vec<OsString> = fs::read_dir(&out_dir)
        .expect("out directory should exist")
        .map(|entry| entry.expect("output entry should be readable").file_name())
        .collect();
    files.sort();
    assert_eq!(
        files,
        [
            "first_moves.csv",
            "matches.csv",
            "matrix_mean.csv",
            "matrix_std.csv",
            "ranking.csv",
        ]
        .map(OsString::from)
    );
    assert_eq!(
        fs::read(out_dir.join("ranking.csv")).expect("ranking output should be readable"),
        ranking
    );
}
