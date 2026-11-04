use std::{fs, path::Path};
use backup_verificado::{Entry, Manifest, Repository};
use tempfile::{tempdir, TempDir};

fn fixture() -> (TempDir, Repository) {
    let temp = tempdir().unwrap();
    fs::create_dir(temp.path().join("source")).unwrap();
    fs::write(temp.path().join("source/a.txt"), b"conteudo").unwrap();
    let repo = Repository::init(temp.path().join("repo")).unwrap();
    (temp, repo)
}

fn count(path: impl AsRef<Path>) -> usize { fs::read_dir(path).unwrap().count() }

#[test]
fn init_refuses_existing_directory() {
    let temp = tempdir().unwrap();
    fs::write(temp.path().join("keep.txt"), b"original").unwrap();
    assert!(Repository::init(temp.path()).is_err());
    assert_eq!(fs::read(temp.path().join("keep.txt")).unwrap(), b"original");
}

#[test]
fn plan_is_read_only_and_counts_unique_objects() {
    let (temp, repo) = fixture();
    fs::write(temp.path().join("source/b.txt"), b"conteudo").unwrap();
    let plan = repo.plan(temp.path().join("source")).unwrap();
    assert_eq!((plan.files, plan.new_objects, plan.reused_objects), (2, 1, 0));
    assert_eq!(plan.total_bytes, 16);
    assert_eq!(plan.new_object_bytes, 8);
    assert_eq!(count(temp.path().join("repo/objects")), 0);
    assert_eq!(count(temp.path().join("repo/manifests")), 0);
    assert!(!temp.path().join("repo/.write-lock").exists());
}

#[test]
fn snapshots_reuse_objects_and_preserve_old_content() {
    let (temp, repo) = fixture();
    let source = temp.path().join("source");
    repo.save(&source, "first").unwrap();
    assert_eq!(repo.plan(&source).unwrap().reused_objects, 1);
    repo.save(&source, "same").unwrap();
    assert_eq!(count(temp.path().join("repo/objects")), 1);
    fs::write(source.join("a.txt"), b"nova versao").unwrap();
    repo.save(&source, "changed").unwrap();
    assert_eq!(count(temp.path().join("repo/objects")), 2);
    repo.restore("first", temp.path().join("old-copy")).unwrap();
    repo.restore("changed", temp.path().join("new-copy")).unwrap();
    assert_eq!(fs::read(temp.path().join("old-copy/a.txt")).unwrap(), b"conteudo");
    assert_eq!(fs::read(temp.path().join("new-copy/a.txt")).unwrap(), b"nova versao");
}

#[test]
fn roundtrip_nested_binary_utf8_and_empty_files() {
    let (temp, repo) = fixture();
    let source = temp.path().join("source");
    fs::create_dir(source.join("estudos")).unwrap();
    fs::write(source.join("estudos/anotacao.txt"), "Fé e engenharia\n").unwrap();
    fs::write(source.join("bytes.bin"), [0, 255, 10, 128]).unwrap();
    fs::write(source.join("empty.txt"), []).unwrap();
    repo.save(&source, "demo").unwrap();
    let destination = temp.path().join("restored");
    let result = repo.restore("demo", &destination).unwrap();
    assert_eq!(result.files, 4);
    assert_eq!(fs::read(destination.join("bytes.bin")).unwrap(), [0, 255, 10, 128]);
    assert_eq!(fs::read_to_string(destination.join("estudos/anotacao.txt")).unwrap(), "Fé e engenharia\n");
    assert_eq!(fs::metadata(destination.join("empty.txt")).unwrap().len(), 0);
}

#[test]
fn duplicate_snapshot_never_replaces_manifest_and_releases_lock() {
    let (temp, repo) = fixture();
    let source = temp.path().join("source");
    repo.save(&source, "demo").unwrap();
    let path = temp.path().join("repo/manifests/demo.json");
    let original = fs::read(&path).unwrap();
    fs::write(source.join("a.txt"), b"alterado").unwrap();
    assert!(repo.save(&source, "demo").is_err());
    assert_eq!(fs::read(path).unwrap(), original);
    assert!(!temp.path().join("repo/.write-lock").exists());
}

#[test]
fn corrupted_object_is_detected_before_restoration() {
    let (temp, repo) = fixture();
    let manifest = repo.save(temp.path().join("source"), "demo").unwrap();
    fs::write(temp.path().join("repo/objects").join(&manifest.entries[0].hash), b"corrompido").unwrap();
    assert!(repo.verify("demo").is_err());
    assert!(repo.restore("demo", temp.path().join("restore")).is_err());
    assert!(!temp.path().join("restore").exists());
}

#[test]
fn restore_refuses_existing_destination_and_repository_paths() {
    let (temp, repo) = fixture();
    repo.save(temp.path().join("source"), "demo").unwrap();
    let destination = temp.path().join("existing");
    fs::create_dir(&destination).unwrap();
    fs::write(destination.join("keep"), b"original").unwrap();
    assert!(repo.restore("demo", &destination).is_err());
    assert_eq!(fs::read(destination.join("keep")).unwrap(), b"original");
    assert!(repo.restore("demo", temp.path().join("repo/restored")).is_err());
}

#[test]
fn rejects_parent_paths_duplicate_entries_and_file_directory_conflicts() {
    let hash = blake3::hash(b"").to_hex().to_string();
    let make = |paths: &[&str]| Manifest {
        format_version: 1, snapshot: "demo".into(), created_unix_seconds: 0,
        entries: paths.iter().map(|path| Entry { path: (*path).into(), bytes: 0, hash: hash.clone() }).collect(),
    };
    for paths in [vec!["../outside"], vec!["/absolute"], vec!["a", "a/b"], vec!["A", "a"], vec!["a\\b"], vec!["C:drive"]] {
        assert!(make(&paths).validate().is_err(), "Caminhos aceitos: {paths:?}");
    }
    assert!(make(&["diretorio/arquivo.txt"]).validate().is_ok());
}

#[test]
fn malformed_or_unsupported_manifests_are_rejected() {
    let (temp, repo) = fixture();
    let path = temp.path().join("repo/manifests/demo.json");
    fs::write(&path, r#"{"format_version":2,"snapshot":"demo","created_unix_seconds":0,"entries":[]}"#).unwrap();
    assert!(repo.read_manifest("demo").is_err());
    fs::write(&path, r#"{"format_version":1,"snapshot":"other","created_unix_seconds":0,"entries":[]}"#).unwrap();
    assert!(repo.read_manifest("demo").is_err());
    fs::write(&path, b"not json").unwrap();
    assert!(repo.read_manifest("demo").is_err());
    assert!(repo.read_manifest("../escape").is_err());
}

#[test]
fn source_and_repository_cannot_overlap() {
    let (temp, repo) = fixture();
    assert!(repo.plan(temp.path()).is_err());
    assert!(repo.plan(temp.path().join("repo")).is_err());
    assert!(repo.plan(temp.path().join("repo/objects")).is_err());
}

#[test]
fn existing_writer_lock_is_preserved_and_blocks_save() {
    let (temp, repo) = fixture();
    let lock = temp.path().join("repo/.write-lock");
    fs::write(&lock, b"outro escritor").unwrap();
    assert!(repo.save(temp.path().join("source"), "demo").is_err());
    assert_eq!(fs::read(lock).unwrap(), b"outro escritor");
    assert_eq!(count(temp.path().join("repo/manifests")), 0);
}

#[test]
fn empty_source_produces_valid_empty_snapshot() {
    let temp = tempdir().unwrap();
    fs::create_dir(temp.path().join("source")).unwrap();
    let repo = Repository::init(temp.path().join("repo")).unwrap();
    repo.save(temp.path().join("source"), "empty").unwrap();
    assert_eq!(repo.verify("empty").unwrap().files, 0);
    repo.restore("empty", temp.path().join("restored")).unwrap();
    assert!(temp.path().join("restored").is_dir());
}

#[cfg(unix)]
#[test]
fn links_are_rejected_without_following_external_content() {
    use std::os::unix::fs::symlink;
    let (temp, repo) = fixture();
    fs::write(temp.path().join("external"), b"fora").unwrap();
    symlink(temp.path().join("external"), temp.path().join("source/link")).unwrap();
    assert!(repo.plan(temp.path().join("source")).is_err());
    assert_eq!(count(temp.path().join("repo/objects")), 0);
}

#[test]
fn cli_runs_the_complete_workflow_and_reports_usage_errors() {
    use std::process::Command;
    let temp = tempdir().unwrap();
    fs::create_dir(temp.path().join("source")).unwrap();
    fs::write(temp.path().join("source/a.txt"), b"cli").unwrap();
    let execute = |args: &[&str]| Command::new(env!("CARGO_BIN_EXE_backup-verificado")).current_dir(temp.path()).args(args).output().unwrap();
    assert!(execute(&["init", "repo"]).status.success());
    assert!(execute(&["plan", "source", "repo"]).status.success());
    assert!(execute(&["save", "source", "repo", "--name", "demo"]).status.success());
    assert!(execute(&["verify", "repo", "--name", "demo"]).status.success());
    assert!(execute(&["restore", "repo", "restored", "--name", "demo"]).status.success());
    assert_eq!(fs::read(temp.path().join("restored/a.txt")).unwrap(), b"cli");
    assert_eq!(execute(&["save"]).status.code(), Some(2));
    assert!(!execute(&["restore", "repo", "restored", "--name", "demo"]).status.success());
}
