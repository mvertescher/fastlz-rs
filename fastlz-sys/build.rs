extern crate cc;

fn main() {
	let mut build = cc::Build::new();
	build.include("fastlz");

	#[cfg(target_os = "linux")]
	build.flag("-Wno-unused-parameter");

	let files = [
        "fastlz/6pack.c",
        "fastlz/6unpack.c",
        "fastlz/fastlz.c",
	];

	build.files(files.iter())
		.compile("fastlz");
	println!("cargo:rustc-link-lib=static=fastlz");
}
