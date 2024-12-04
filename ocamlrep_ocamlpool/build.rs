// Copyright (c) Meta Platforms, Inc. and affiliates.

// Assume an opam environment (`eval "$(opam env --switch=default
// --set-switch)"`) then to find the prevailing standard library caml
// headers, `OCAMLLIB=$(ocamlopt.opt -config | grep standard_library:
// | awk '{ print $2 }')`.
fn ocamllib_dir() -> std::path::PathBuf {
    let mut sh = std::process::Command::new("sh");
    sh.args(["-c", "pwd"]);
    let bytes = sh.output().unwrap().stdout;
    let str_output = std::str::from_utf8(&bytes).unwrap().trim();
    println!("Current directory: {}", str_output);

    let mut sh = std::process::Command::new("sh");
    sh.args(["-c", "opam --version"]);
    let bytes = sh.output().unwrap().stdout;
    let str_output = std::str::from_utf8(&bytes).unwrap().trim();
    println!("Opam version {}", str_output);

    let mut sh = std::process::Command::new("sh");
    sh.args([
        "-c",
        "ocamlopt.opt -config | grep standard_library: | awk '{ print $2 }'",
    ]);
    let bytes = sh.output().unwrap().stdout;
    let str_output = std::str::from_utf8(&bytes).unwrap().trim();
    println!("Proposed path output: {}", str_output);
    let proposed_path = std::path::Path::new(str_output);
    // A supercaml 'ocamlopt.opt' can report standard library paths that don't
    // exist.
    if proposed_path.exists() {
        proposed_path.to_path_buf()
    } else {
        // Fallback to guessing the location given knowledge of where
        // 'ocamlopt.opt' itself it.
        let mut sh = std::process::Command::new("sh");
        sh.args(["-c", "which ocamlopt.opt"]);
        let bytes = sh.output().unwrap().stdout;
        let str_output = std::str::from_utf8(&bytes).unwrap().trim();
        println!("Output: {}", str_output);
        std::path::Path::new(str_output)
            .ancestors()
            .nth(2)
            .unwrap()
            .join("lib/ocaml")
    }
}

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=ocamlpool.c");
    cc::Build::new()
        .include(ocamllib_dir().as_path().to_str().unwrap())
        .file("ocamlpool.c")
        .compile("ocamlpool");
}
