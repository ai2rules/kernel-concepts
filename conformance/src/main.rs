//! Runs the conformance kit in `kit/` against a released `harness` binary.
//!
//! Each case is a gate request and the verdict the kernel gives it. This sends the
//! request to `harness gate --world <world>` and compares `decision`, `rule`, the
//! post-call `context.taint`, and `context.flow` where the case states one. The kit
//! comes from the kernel release named in `KERNEL_VERSION`; the binary should be the
//! npm release of the same version (`npm install -g ai2rules-harness@<version>`).
//!
//!     cargo run -p conformance -- [--harness <path>] [--kit <dir>]

use serde_json::Value;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

struct Args {
    harness: String,
    kit: PathBuf,
}

fn args() -> Args {
    let mut out = Args {
        harness: "harness".into(),
        kit: PathBuf::from("kit"),
    };
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--harness" => out.harness = it.next().expect("--harness needs a path"),
            "--kit" => out.kit = PathBuf::from(it.next().expect("--kit needs a directory")),
            "-h" | "--help" => {
                println!("conformance [--harness <path>] [--kit <dir>]");
                std::process::exit(0);
            }
            other => panic!("unknown argument {other}"),
        }
    }
    out
}

/// One `harness gate` call: the request on stdin, the response on stdout.
fn gate(harness: &str, world: &Path, request: &Value) -> Result<Value, String> {
    let mut child = Command::new(harness)
        .args(["gate", "--world"])
        .arg(world)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("cannot run {harness}: {e}"))?;
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(request.to_string().as_bytes())
        .map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    serde_json::from_slice(&out.stdout).map_err(|e| {
        format!(
            "no JSON from harness gate ({e}); stderr: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        )
    })
}

fn load_cases(text: &str) -> Vec<Value> {
    let doc: Value = serde_yaml::from_str(text).expect("cases file parses");
    doc["cases"].as_array().expect("a `cases` list").clone()
}

/// Compare what the kernel said with what the case expects. Returns the mismatches.
fn compare(expect: &Value, got: &Value) -> Vec<String> {
    let mut diffs = Vec::new();
    let mut check = |what: &str, want: &Value, have: &Value| {
        if want != have {
            diffs.push(format!("{what}: expected {want}, got {have}"));
        }
    };
    check("decision", &expect["decision"], &got["decision"]);
    check("rule", &expect["rule"], &got["rule"]);
    check("taint", &expect["taint"], &got["context"]["taint"]);
    if !expect["flow"].is_null() {
        check("flow", &expect["flow"], &got["context"]["flow"]);
    }
    diffs
}

/// Run one world and its cases; returns (passed, failed).
fn run_set(harness: &str, label: &str, world: &Path, cases: &[Value]) -> (usize, usize) {
    println!("\n== {label} ({} cases) ==", cases.len());
    let (mut ok, mut bad) = (0, 0);
    for case in cases {
        let name = case["name"].as_str().unwrap_or("<unnamed>");
        match gate(harness, world, &case["request"]) {
            Ok(got) => {
                let diffs = compare(&case["expect"], &got);
                if diffs.is_empty() {
                    ok += 1;
                    println!("ok    {name}: {}", got["decision"].as_str().unwrap_or("?"));
                } else {
                    bad += 1;
                    println!("FAIL  {name}");
                    for d in diffs {
                        println!("        {d}");
                    }
                }
            }
            Err(e) => {
                bad += 1;
                println!("FAIL  {name}: {e}");
            }
        }
    }
    (ok, bad)
}

/// The directory the roots cases describe: `{project}` with `src/`, `inbox/`,
/// `vendor/` (with a `.env`), `private/` (with a key) and `link -> private`.
#[cfg(unix)]
fn roots_project() -> PathBuf {
    let base = std::env::temp_dir().join(format!("ai2rules-conformance-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    for sub in ["src", "inbox", "vendor", "private", "untracked"] {
        std::fs::create_dir_all(base.join(sub)).expect("create fixture dir");
    }
    let root = std::fs::canonicalize(&base).expect("canonical fixture root");
    for (file, body) in [
        ("src/lib.rs", "// code\n"),
        ("inbox/message.txt", "untrusted\n"),
        ("vendor/dep.js", "// dep\n"),
        ("vendor/.env", "SECRET=1\n"),
        ("private/id_rsa", "KEY\n"),
    ] {
        std::fs::write(root.join(file), body).expect("write fixture file");
    }
    std::os::unix::fs::symlink(root.join("private"), root.join("link")).expect("fixture symlink");
    root
}

fn main() {
    let a = args();
    let kit = a.kit.join("one-kernel");
    let read =
        |f: &str| std::fs::read_to_string(kit.join(f)).unwrap_or_else(|e| panic!("read {f}: {e}"));
    let (mut ok, mut bad) = (0, 0);

    for (label, world, cases) in [
        ("demo world", "demo-world.yaml", "cases.yaml"),
        ("flow contracts", "flow-world.yaml", "flow-cases.yaml"),
    ] {
        let (o, b) = run_set(
            &a.harness,
            label,
            &kit.join(world),
            &load_cases(&read(cases)),
        );
        ok += o;
        bad += b;
    }

    #[cfg(unix)]
    {
        let project = roots_project();
        let p = project.to_str().expect("utf-8 path");
        let world = project.join("roots-world.yaml");
        std::fs::write(&world, read("roots-world.yaml").replace("{project}", p))
            .expect("write roots world");
        let cases = load_cases(&read("roots-cases.yaml").replace("{project}", p));
        let (o, b) = run_set(&a.harness, "path scope (roots)", &world, &cases);
        ok += o;
        bad += b;
        let _ = std::fs::remove_dir_all(&project);
    }
    #[cfg(not(unix))]
    println!("\n(path-scope cases skipped: their fixture needs a Unix symlink)");

    println!("\n{ok} passed, {bad} failed");
    if bad > 0 {
        std::process::exit(1);
    }
}
