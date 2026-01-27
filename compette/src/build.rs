use std::{path::PathBuf, process::Command};

const START_FILE: &[u8] = include_bytes!("../start_x86.s");

fn create_start_file(path: PathBuf) -> Result<PathBuf, std::io::Error> {
    let _ = std::fs::write(&path, START_FILE);
    let gas_build = Command::new("as")
        .arg("-o")
        .arg(&path.with_extension("o"))
        .arg(&path)
        .output();

    gas_build.unwrap();

    Ok(path.with_extension("o"))
}

pub fn build(source: PathBuf) -> Result<PathBuf, ()> {
    let start_file = create_start_file(source.with_file_name("start").with_extension("s"));
    let out_file = PathBuf::from(".build/").join(source.file_stem().unwrap());

    let clang_build = Command::new("ld")
        .arg(start_file.unwrap())
        .arg(&source)
        .arg("-o")
        .arg(&out_file)
        .output();

    if let Ok(output) = clang_build {
        if output.status.success() {
            Ok(out_file)
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
