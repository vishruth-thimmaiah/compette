use std::{fs, path::PathBuf, process::Command};

fn output_path() -> PathBuf {
    let path = PathBuf::from(".build/");
    if !path.exists() {
        fs::create_dir(&path).unwrap();
    }
    path
}

fn llvm_ir_path() -> PathBuf {
    let path = output_path().join("ir/");
    if !path.exists() {
        fs::create_dir(&path).unwrap();
    }
    path
}

fn output_binary_path() -> PathBuf {
    output_path().join("output")
}

pub fn build(source: PathBuf, ir: String) -> Result<PathBuf, ()> {
    let dir = llvm_ir_path();
    let path = dir.join(source.file_stem().unwrap()).with_extension("ll");
    let output_path = output_binary_path();

    fs::write(&path, ir).unwrap();

    let clang_build = Command::new("clang")
        .arg(path)
        .arg(".build/stdlib.a")
        .arg("-o")
        .arg(&output_path)
        .output();

    if let Ok(output) = clang_build {
        if output.status.success() {
            Ok(output_path)
        } else {
            eprintln!(
                "> Error [{}] while building the binary:\n{}",
                output.status,
                std::str::from_utf8(&output.stderr).unwrap()
            );
            Err(())
        }
    } else {
        eprintln!("> Error running clang to build the binary.");
        Err(())
    }
}

pub fn run(output: std::path::PathBuf) {
    let output = std::process::Command::new(output)
        .status()
        .unwrap()
        .code()
        .unwrap();
    println!("Exit Code: {}", output);
}
