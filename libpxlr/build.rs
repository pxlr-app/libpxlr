use std::env;

fn main() {
	let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let profile = env::var("PROFILE").unwrap();

	cbindgen::Builder::new()
		.with_crate(crate_dir)
		.generate()
		.expect("Unable to generate bindings")
		.write_to_file(format!("../target/{}/libpxlr.h", profile));
}
