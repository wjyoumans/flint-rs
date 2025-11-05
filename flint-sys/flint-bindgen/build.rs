extern crate bindgen;

use std::env;
use std::fs;
use std::path::PathBuf;

// Headers to skip
const SKIP_HEADERS: &[&str] = &[
    "NTL-interface.h",
    "crt_helpers.h",
    "fft_small.h",
    "gettimeofday.h",
    "longlong_asm_clang.h",
    "longlong_asm_gcc.h",
    "longlong_asm_gnu.h",
    "longlong_div_gnu.h",
    "longlong_msc_arm64.h",
    "longlong_msc_x86.h",
    "machine_vectors.h",
    "profiler.h",
];

fn get_flint_headers(include_path: &PathBuf) -> Vec<String> {
    let flint_headers_dir = include_path.join("flint");
    let mut headers = Vec::new();

    if let Ok(entries) = fs::read_dir(flint_headers_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    if let Some(name) = file_name.to_str() {
                        // Check if it's a .h file and not in skip list
                        if name.ends_with(".h") && !SKIP_HEADERS.contains(&name) {
                            headers.push(name.to_string());
                        }
                    }
                }
            }
        }
    }

    headers.sort();
    headers
}

fn get_imports(header: &str, include_path: &PathBuf) -> String {
    let headers = get_flint_headers(include_path)
        .into_iter()
        .filter(|h| h != header);
    let mut out = String::from("use libc::*;\n");
    out += "use crate::deps::*;\n";
    out += "use crate::bindgen::*;\n";

    for h in headers {
        let temp = h
            .split(".")
            .next()
            .expect("Error making import statement")
            .replace("-", "_");
        out += &format!("use crate::{}::*;\n", temp);
    }
    return out;
}

fn generate_bindings(header: &str, include_path: &PathBuf, out_path: &PathBuf) {
    let include_arg = format!("-I{}", include_path.display());
    let include_fp = include_path.join("flint").join(header);

    let mut out_fp = out_path.join(header);
    let mut extern_out_fp = out_path.join("extern").join(header);
    out_fp.set_extension("rs");
    extern_out_fp.set_extension("c");

    let imports = get_imports(header, include_path);

    let bindings = bindgen::Builder::default()
        .header(include_fp.to_string_lossy())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .wrap_static_fns(true)
        .wrap_static_fns_path(extern_out_fp)
        .allowlist_file(include_fp.to_string_lossy())
        .allowlist_recursively(false)
        .clang_arg(include_arg)
        .raw_line(imports)
        .generate_cstr(true)
        .ctypes_prefix("libc")
        .c_naming(false)
        .derive_debug(true)
        .derive_copy(true)
        .derive_default(true)
        .merge_extern_blocks(true)
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .blocklist_function("flint_vprintf")
        .blocklist_function("flint_vfprintf")
        .generate()
        .expect(&format!("Unable to generate bindings for {}", header));

    bindings
        .write_to_file(out_fp)
        .expect(&format!("Unable to write bindings for {}", header));
}

fn main() {
    println!("cargo:rustc-link-lib=flint");

    // Use INCLUDE_DIR env variable to pass flint include dir if needed
    let include_path = PathBuf::from(
        env::var("INCLUDE_DIR").expect("Environment variable INCLUDE_DIR is not defined."),
    );
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    for h in get_flint_headers(&include_path) {
        generate_bindings(&h, &include_path, &out_path);
    }
}
