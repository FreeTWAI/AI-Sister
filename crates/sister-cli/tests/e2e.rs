use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn sister(data_dir: &Path, config: Option<&Path>, args: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_sister"));
    command.arg("--data-dir").arg(data_dir);
    if let Some(config) = config {
        command.arg("--config").arg(config);
    }
    command.args(args);
    command.output().expect("run sister")
}

#[test]
fn test_e2e_core_workflows() {
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "sister-e2e-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp data dir");

    // We can run `doctor` to check basic system checks. This includes install lifecycle checks.
    let output = sister(&dir, None, &["doctor"]);
    assert!(output.status.success(), "doctor failed");

    // Check consent.
    let output = sister(&dir, None, &["consent", "--grant", "local-recording"]);
    assert!(output.status.success(), "consent local-recording failed");

    let output = sister(&dir, None, &["consent", "--grant", "cloud-reading"]);
    assert!(output.status.success(), "consent cloud-reading failed");

    // Try prune
    let output = sister(&dir, None, &["prune"]);
    assert!(output.status.success(), "prune failed");

    // Try stop-all
    let output = sister(&dir, None, &["stop-all"]);
    assert!(output.status.success(), "stop-all failed");

    // Try url-policy
    let output = sister(&dir, None, &["url-policy"]);
    assert!(output.status.success(), "url-policy failed");

    let _ = std::fs::remove_dir_all(&dir);
}
