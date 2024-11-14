use std::{fs, path::PathBuf, process::Command};

use inkwell::module::Module;

fn run_binary() {
    let path = PathBuf::from(".build/output");
    let run_cmd = Command::new(path).status();

    if let Ok(output) = run_cmd {
        println!("\n> Binary ran with {}", output)
    } else {
        panic!("\n> Error running the binary.")
    }
}

pub fn build_ir(module: &Module, run: bool) {
    let path = PathBuf::from(".build/llvm-ir/");
    let _ = fs::create_dir_all(&path);

    let mod_name = module.get_name().to_str().unwrap();
    let path = &path
        .join("llvm-ir/")
        .with_file_name(mod_name)
        .with_extension("ll");
    module.print_to_file(path).unwrap();
    let clang_build = Command::new("clang")
        .arg(path)
        .arg(".build/stdlib.a")
        .arg("-o")
        .arg(".build/output")
        .output();

    if let Ok(output) = clang_build {
        if output.status.success() {
            println!("> Binary built.\n")
        } else {
            panic!(
                "> Error while building the binary: {}",
                std::str::from_utf8(&output.stderr).unwrap()
            )
        }
    } else {
        panic!("> Error running clang to build the binary.")
    }
    if run {
        run_binary();
    }
}
