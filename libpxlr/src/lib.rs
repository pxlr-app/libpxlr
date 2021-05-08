#[no_mangle]
pub extern "C" fn pxlr_test(ptr: *const u8, len: usize) -> *mut String {
	let name = unsafe {
		let slice = ::std::slice::from_raw_parts(ptr, len);
		::std::str::from_utf8_unchecked(slice)
	};
	let mut name = name.to_string();
	name.push_str(" world!");
	Box::into_raw(Box::new(name))
}
