#[no_mangle]
pub extern "C" fn pxlr_file_reader_create(
	file_path: *mut std::os::raw::c_char,
) -> *mut async_std::fs::File {
	let cstring = unsafe { std::ffi::CString::from_raw(file_path) };
	#[cfg(target_os = "windows")]
	let path = Into::<std::ffi::OsString>::into(cstring.into_string().unwrap());
	#[cfg(target_os = "windows")]
	let path: &std::path::Path = path.as_ref();
	#[cfg(not(target_os = "windows"))]
	let path = <std::ffi::OsStr as std::os::unix::ffi::OsStrExt>::from_bytes(cstring.as_bytes());
	#[cfg(target_os = "windows")]
	let path: &std::path::Path = path.as_ref();
	match async_std::task::block_on(async_std::fs::OpenOptions::new().read(true).open(path)) {
		Err(_) => std::ptr::null_mut(),
		Ok(file) => Box::into_raw(Box::new(file)),
	}
}

#[no_mangle]
pub extern "C" fn pxlr_file_reader_close(file_reader_handle: *mut async_std::fs::File) {
	if !file_reader_handle.is_null() {
		unsafe {
			Box::from_raw(file_reader_handle);
		}
	}
}

#[no_mangle]
pub extern "C" fn pxlr_file_writer_create(
	file_path: *mut std::os::raw::c_char,
) -> *mut async_std::fs::File {
	let cstring = unsafe { std::ffi::CString::from_raw(file_path) };
	#[cfg(target_os = "windows")]
	let path = Into::<std::ffi::OsString>::into(cstring.into_string().unwrap());
	#[cfg(target_os = "windows")]
	let path: &std::path::Path = path.as_ref();
	#[cfg(not(target_os = "windows"))]
	let path = <std::ffi::OsStr as std::os::unix::ffi::OsStrExt>::from_bytes(cstring.as_bytes());
	#[cfg(target_os = "windows")]
	let path: &std::path::Path = path.as_ref();
	match async_std::task::block_on(async_std::fs::OpenOptions::new().write(true).open(path)) {
		Err(_) => std::ptr::null_mut(),
		Ok(file) => Box::into_raw(Box::new(file)),
	}
}

#[no_mangle]
pub extern "C" fn pxlr_file_writer_close(file_writer_handle: *mut async_std::fs::File) {
	if !file_writer_handle.is_null() {
		unsafe {
			Box::from_raw(file_writer_handle);
		}
	}
}
