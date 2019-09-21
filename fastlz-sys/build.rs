extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
	let mut build = cc::Build::new();
	build.include("fastlz");

	#[cfg(target_os = "linux")]
	build.flag("-Wno-unused-parameter");

	let files = [
        "fastlz/fastlz.c",
	];

	build.files(files.iter())
		.compile("fastlz");
	println!("cargo:rustc-link-lib=static=fastlz");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("fastlz/fastlz.h")
        .use_core()
        .ctypes_prefix("cty")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!")
}
