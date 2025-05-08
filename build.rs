use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/bindings.h");
    println!("cargo:rerun-if-changed=src/gl.rs");

    let gl_pkg = pkg_config::Config::new()
        .print_system_libs(false)
        .probe("gl")
        .expect("Failed to find OpenGL pkg-config");

    let builder = bindgen::Builder::default()
        .header("src/bindings.h")
        .clang_args(
            gl_pkg
                .include_paths
                .iter()
                .map(|p| format!("-I{}", p.display())),
        )
        .allowlist_function("gl(Begin|End|Vertex2f|Color3f|LineWidth|Enable|Disable)")
        .allowlist_var("GL_(BLEND|TEXTURE_2D|LINE_LOOP)")
        .generate_comments(false);

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
