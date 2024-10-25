use std::{fs, path::PathBuf, process::Command};

use inkwell::module::Module;

fn rust_stdlib_path() -> String {
    #[cfg(target_os = "linux")]

    let home = Command::new("rustup").arg("show").arg("home").output().unwrap().stdout;

    let rust_lib_path = format!(
        "{}/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/",
        std::str::from_utf8(&home.get(..home.len() - 1).unwrap()).unwrap()
    );
    for file in fs::read_dir(rust_lib_path).unwrap() {
        let file = file.unwrap();
        let path = file.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.starts_with("libstd-") && file_name.ends_with(".so") {
                return path.to_str().unwrap().to_string();
            }
        }
    }
    panic!("Could not find the Rust stdlib path.")
}

pub fn build_ir(module: &Module) {
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
        .arg(rust_stdlib_path())
        .output();

    if let Ok(output) = clang_build {
        if output.status.success() {
            println!("Binary built.")
        } else {
            panic!(
                "Error while building the binary: {}",
                std::str::from_utf8(&output.stderr).unwrap()
            )
        }
    } else {
        panic!("Error running clang to build the binary.")
    }
}
