#![cfg(feature = "cli")]

use std::io::Write;
use std::process::Command;

use rust_xlsxwriter::Workbook;
use tempfile::tempdir;

fn write_tiny_xlsx(path: &std::path::Path, sheet_name: &str) {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_name(sheet_name).unwrap();

    worksheet.write(0, 0, "x1").unwrap();
    worksheet.write(0, 1, "x2").unwrap();
    worksheet.write(0, 2, "y").unwrap();

    for i in 0..15usize {
        let row = i + 1;
        let x1 = i as f64;
        let x2 = (i as f64) * 0.5;
        let y = 1.5 * x1 - 0.7 * x2 + 0.1;
        worksheet.write(row as u32, 0, x1).unwrap();
        worksheet.write(row as u32, 1, x2).unwrap();
        worksheet.write(row as u32, 2, y).unwrap();
    }

    workbook.save(path).unwrap();
}

#[test]
fn symreg_runs_on_tiny_csv() {
    let dir = tempdir().unwrap();
    let csv_path = dir.path().join("tiny.csv");
    let mut f = std::fs::File::create(&csv_path).unwrap();
    writeln!(f, "x1,x2,y").unwrap();
    for i in 0..25 {
        let x1 = i as f64;
        let x2 = (i as f64) * 0.5;
        let y = 1.5 * x1 - 0.7 * x2 + 0.1;
        writeln!(f, "{x1},{x2},{y}").unwrap();
    }
    drop(f);

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_symreg"));
    cmd.arg(&csv_path)
        .arg("--y")
        .arg("y")
        .arg("--seed=0")
        .arg("--niterations=1")
        .arg("--populations=1")
        .arg("--population-size=20")
        .arg("--ncycles-per-iteration=5")
        .arg("--tournament-selection-n=3")
        .arg("--no-progress")
        .arg("--no-should-optimize-constants");

    let out = cmd.output().unwrap();
    assert!(
        out.status.success(),
        "symreg failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("target: y"));
    assert!(stdout.contains("complexity"));
    assert!(stdout.contains("equation"));
}

#[test]
fn symreg_list_operators_prints_registry() {
    let out = Command::new(env!("CARGO_BIN_EXE_symreg"))
        .arg("--list-operators")
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("--unary-operators:"));
    assert!(stdout.contains("--binary-operators:"));
}

#[test]
fn symreg_writes_json_output() {
    let dir = tempdir().unwrap();
    let csv_path = dir.path().join("out.csv");
    let json_path = dir.path().join("out.json");
    let mut f = std::fs::File::create(&csv_path).unwrap();
    writeln!(f, "x1,x2,y").unwrap();
    for i in 0..15 {
        let x1 = i as f64;
        let x2 = (i as f64) * 0.5;
        let y = 1.5 * x1 - 0.7 * x2 + 0.1;
        writeln!(f, "{x1},{x2},{y}").unwrap();
    }
    drop(f);

    let out = Command::new(env!("CARGO_BIN_EXE_symreg"))
        .arg(&csv_path)
        .arg("--y")
        .arg("y")
        .arg("--seed=0")
        .arg("--niterations=1")
        .arg("--populations=1")
        .arg("--population-size=20")
        .arg("--ncycles-per-iteration=5")
        .arg("--tournament-selection-n=3")
        .arg("--no-progress")
        .arg("--no-should-optimize-constants")
        .arg("--output")
        .arg(&json_path)
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "symreg failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let json = std::fs::read_to_string(&json_path).unwrap();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(v.is_array());
}

#[test]
fn symreg_runs_on_tiny_xlsx() {
    let dir = tempdir().unwrap();
    let xlsx_path = dir.path().join("tiny.xlsx");
    write_tiny_xlsx(&xlsx_path, "Sheet1");

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_symreg"));
    cmd.arg(&xlsx_path)
        .arg("--sheet")
        .arg("Sheet1")
        .arg("--y")
        .arg("y")
        .arg("--seed=0")
        .arg("--niterations=1")
        .arg("--populations=1")
        .arg("--population-size=20")
        .arg("--ncycles-per-iteration=5")
        .arg("--tournament-selection-n=3")
        .arg("--no-progress")
        .arg("--no-should-optimize-constants");

    let out = cmd.output().unwrap();
    assert!(
        out.status.success(),
        "symreg failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("target: y"));
}

#[test]
fn symreg_accepts_operator_flags() {
    let dir = tempdir().unwrap();
    let csv_path = dir.path().join("ops.csv");
    let mut f = std::fs::File::create(&csv_path).unwrap();
    writeln!(f, "x1,x2,y").unwrap();
    for i in 0..20 {
        let x1 = i as f64;
        let x2 = (i as f64) * 0.5;
        let y = 1.5 * x1 - 0.7 * x2 + 0.1;
        writeln!(f, "{x1},{x2},{y}").unwrap();
    }
    drop(f);

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_symreg"));
    cmd.arg(&csv_path)
        .arg("--y")
        .arg("y")
        .arg("--seed=0")
        .arg("--niterations=1")
        .arg("--populations=1")
        .arg("--population-size=20")
        .arg("--ncycles-per-iteration=5")
        .arg("--tournament-selection-n=3")
        .arg("--unary-operators=-")
        .arg("--binary-operators=+,*")
        .arg("--no-progress")
        .arg("--no-should-optimize-constants");

    let out = cmd.output().unwrap();
    assert!(
        out.status.success(),
        "symreg failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_symreg"));
    cmd.arg(&csv_path)
        .arg("--y")
        .arg("y")
        .arg("--unary-operators=not_a_real_op")
        .arg("--no-progress");
    let out = cmd.output().unwrap();
    assert!(!out.status.success());
}
