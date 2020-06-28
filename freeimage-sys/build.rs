#[cfg(windows)]
extern crate cc;
extern crate bindgen;
extern crate fs_extra;
extern crate regex;

use std::process::Command;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::{PathBuf, Path};
use std::mem::drop;
use regex::{Regex, NoExpand};

#[cfg(target_os="macos")]
use std::ffi::OsString;
#[cfg(target_os="macos")]
use std::os::unix::ffi::OsStringExt;

#[cfg(target_os="macos")]
fn build_macos() {
	let freeimage_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let freeimage_native_dir = Path::new(&freeimage_dir).join("FreeImage");
    let out_dir = env::var("OUT_DIR").unwrap();
	let freeimage_copy = Path::new(&out_dir).join("FreeImage");
	drop(fs_extra::dir::remove(&freeimage_copy));
	fs_extra::dir::copy(freeimage_native_dir, &out_dir, &fs_extra::dir::CopyOptions::new()).unwrap();
	let xcode_select_out: OsString = OsString::from_vec(Command::new("xcode-select")
                .arg("-print-path")
                .output().unwrap()
		        .stdout);
    let xcode_path = xcode_select_out.into_string().unwrap();
	let xcode_path = xcode_path.lines().next().unwrap();
    let sdks_path = Path::new(&xcode_path).join("Platforms/MacOSX.platform/Developer/SDKs");
    let last_sdk_entry = match fs::read_dir(&sdks_path){
        Ok(sdks) => sdks.last().unwrap().unwrap(),
        Err(_) => panic!("Couldn't find SDK at {}, probably xcode is not installed",sdks_path.to_str().unwrap())
    };

    let sdk = last_sdk_entry.path().as_path().file_stem().unwrap().to_str().unwrap().to_string();
    if sdk.contains("MacOSX"){
        let version = &sdk[6..];
        let output = Command::new("make")
		    .current_dir(&freeimage_copy)
		    .env("MACOSX_SDK",version)
		    .arg("-j4")
		    .output()
			.unwrap();
		
		if !output.status.success(){
			panic!("{}", String::from_utf8(output.stdout).unwrap());
		}

	    let out_dir = env::var("OUT_DIR").unwrap();
	    let dest_path = Path::new(&out_dir).join("libfreeimage.a");
	    fs::copy(freeimage_copy.join("libfreeimage.a"),dest_path).unwrap();
	    println!("cargo:rustc-flags= -L native={}",out_dir);

    }else{
        panic!("Couldn't find SDK at {}, probably xcode is not installed",sdks_path.to_str().unwrap())
    }
}

fn build_linux() {
	let freeimage_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let freeimage_native_dir = Path::new(&freeimage_dir).join("FreeImage");
    let out_dir = env::var("OUT_DIR").unwrap();
	let freeimage_copy = Path::new(&out_dir).join("FreeImage");
	drop(fs_extra::dir::remove(&freeimage_copy));
	fs_extra::dir::copy(freeimage_native_dir, &out_dir, &fs_extra::dir::CopyOptions::new()).unwrap();
    let output = Command::new("make")
	    .current_dir(&freeimage_copy)
	    .arg("-j4")
	    .output()
		.unwrap();
		
	if !output.status.success(){
		panic!("{}", String::from_utf8(output.stdout).unwrap());
	}
	
    let dest_path = Path::new(&out_dir).join("libfreeimage.a");
    fs::copy(freeimage_copy.join("Dist/libfreeimage.a"), dest_path).unwrap();
    println!("cargo:rustc-flags= -L native={}",out_dir);
}

fn build_emscripten() {
	let freeimage_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let freeimage_native_dir = Path::new(&freeimage_dir).join("FreeImage");
    let out_dir = env::var("OUT_DIR").unwrap();
	let freeimage_copy = Path::new(&out_dir).join("FreeImage");
	drop(fs_extra::dir::remove(&freeimage_copy));
	fs_extra::dir::copy(freeimage_native_dir, &out_dir, &fs_extra::dir::CopyOptions::new()).unwrap();
    let output = Command::new("emmake")
		.arg("make")
	    .current_dir(&freeimage_copy)
	    .arg("-j4")
	    .output()
		.unwrap();
		
	if !output.status.success(){
		panic!("{}", String::from_utf8(output.stdout).unwrap());
	}
		
    let dest_path = Path::new(&out_dir).join("libfreeimage.a");
    fs::copy(freeimage_copy.join("Dist/libfreeimage.a"),dest_path).unwrap();
    println!("cargo:rustc-flags= -L native={}",out_dir);
}

// #[cfg(windows)]
// fn retarget_ms_proj(target: &str, proj: &str, freeimage_native_dir: &PathBuf){
// 	let mut devenv = cc::windows_registry::find(target, "devenv.exe")
// 		.expect("Couldn't find devenv, perhaps you need to install visual studio?");

// 	let output = devenv
// 		.arg(proj)
// 		.arg("/upgrade")
// 		.current_dir(&freeimage_native_dir)
// 		.output()
// 		.expect("Couldn't Freeiamge visual studio update solution");

// 	if !output.status.success(){
// 		panic!("{}", String::from_utf8(output.stdout).unwrap());
// 	}
// }

#[cfg(windows)]
fn build_windows(target: &str) {
	let freeimage_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let freeimage_native_dir = Path::new(&freeimage_dir).join("FreeImage");
    let out_dir = env::var("OUT_DIR").unwrap();
	let freeimage_copy = Path::new(&out_dir).join("FreeImage");
	drop(fs_extra::dir::remove(&freeimage_copy));
	fs_extra::dir::copy(freeimage_native_dir, &out_dir, &fs_extra::dir::CopyOptions::new()).unwrap();
	let freeimage_proj = "FreeImage.2017.sln";

	// retarget_ms_proj(target, freeimage_proj, &freeimage_copy);

	let mut msbuild = cc::windows_registry::find(target, "msbuild.exe")
		.expect("Couldn't find msbuild, perhaps you need to install visual studio?");

	#[cfg(debug_assertions)]
	let config = "Debug";

	#[cfg(not(debug_assertions))]
	let config = "Release";

	let platform = if target.contains("x86_64") {
		"x64"
	} else if target.contains("thumbv7a") {
		"arm"
	} else if target.contains("aarch64") {
		"ARM64"
	} else if target.contains("i686") {
		"x86"
	} else {
		panic!("unsupported msvc target: {}", target);
	};

	let output = msbuild.arg(freeimage_proj)
		.arg(&format!("-property:Configuration={}", config))
		.arg(&format!("-property:Platform={}", platform))
		.current_dir(&freeimage_copy)
		.output()
		.unwrap();

	if !output.status.success(){
		panic!("{}", String::from_utf8(output.stdout).unwrap());
	}

	#[cfg(debug_assertions)]
	let libname = "FreeImaged";

	#[cfg(not(debug_assertions))]
	let libname = "FreeImage";

	let out_dir = env::var("OUT_DIR").unwrap();
	let src_dir = freeimage_copy
		.join(platform)
		.join(config);

	let lib_name = format!("{}.lib", libname);
	let dst_path = Path::new(&out_dir).join(&lib_name);
	let src_path = src_dir.join(&lib_name);
	fs::copy(src_path, dst_path).unwrap();

	let dll_name = format!("{}.dll", libname);
	let dst_path = Path::new(&out_dir).join(&dll_name);
	let src_path = src_dir.join(&dll_name);
	fs::copy(src_path, dst_path).unwrap();

	println!("cargo:rustc-flags= -L native={}",out_dir);
}

fn generate_binding() {
	// Tell cargo to invalidate the built crate whenever the wrapper changes
	println!("cargo:rerun-if-changed=wrapper.h");

	// println!("cargo:rustc-link-lib=static=stdc++");

	// The bindgen::Builder is the main entry point
	// to bindgen, and lets you build up options for
	// the resulting bindings.
	let bindings = bindgen::Builder::default()
		// The input header we would like to generate
		// bindings for.
		.header("wrapper.h")

		.opaque_type("std::*")

		// Tell cargo to invalidate the built crate whenever any of the
		// included header files changed.
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		// Finish the builder and generate the bindings.
		.generate()
		// Unwrap the Result and panic on failure.
		.expect("Unable to generate bindings");

	// Write the bindings to the $OUT_DIR/bindings.rs file.
	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	// bindings
	// 	.write_to_file(out_path.join("bindings.rs"))
	// 	.expect("Couldn't write bindings!");
	let mut content = bindings.to_string();
	let re = Regex::new("pub type (FREE_IMAGE_FORMAT|FREE_IMAGE_TYPE|FREE_IMAGE_COLOR_TYPE|FREE_IMAGE_QUANTIZE|FREE_IMAGE_DITHER|FREE_IMAGE_JPEG_OPERATION|FREE_IMAGE_TMO|FREE_IMAGE_FILTER|FREE_IMAGE_COLOR_CHANNEL|FREE_IMAGE_MDTYPE|FREE_IMAGE_MDMODEL) = ::std::os::raw::c_int;\n").unwrap();
	content = re.replace_all(&content[..], NoExpand("")).into_owned();
	let mut file = fs::File::create(out_path.join("bindings.rs")).expect("Couldn't not create bindings.rs!");
	file.write_all(content.as_bytes()).expect("Couldn't write bindings!");
}

fn build() {
	let target_triple = env::var("TARGET").unwrap();
	if target_triple.contains("linux") {
		build_linux()
	}else if target_triple.contains("darwin") {
		#[cfg(target_os="macos")]
		build_macos()
	}else if target_triple.contains("emscripten") {
		build_emscripten()
	}else if target_triple.contains("windows"){
		#[cfg(windows)]
		build_windows(&target_triple)
	}else{
		panic!("target OS {} not suported yet", target_triple);
	}
	println!("cargo:rustc-link-lib=static=freeimage");
	println!("cargo:rustc-link-search=native=/usr/lib/gcc/x86_64-linux-gnu/10");
	println!("cargo:rustc-link-lib=dylib=stdc++");
}

fn main(){
	generate_binding();
	build();
}