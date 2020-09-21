use freeimage::*;
use std::ffi::CString;
use std::path::Path;

fn main() {
	// https://github.com/arturoc/freeimage-rs/blob/master/src/lib.rs
	unsafe {
		FreeImage_Initialise(0);

		let filename_in = Path::new("/mnt/c/Developments/photo.jpg");
		let ptr = {
			let cname = CString::new(filename_in.to_str().unwrap().as_bytes()).unwrap();
			let ptr = FreeImage_Load(FREE_IMAGE_FORMAT_FIF_JPEG, cname.as_ptr(), 0);
			if ptr.is_null() {
				panic!("Could not load image");
			} else {
				ptr
			}
		};
		{
			let filename_out = Path::new("/mnt/c/Developments/photo_webp.webp");

			let cname = CString::new(filename_out.to_str().unwrap().as_bytes()).unwrap();
			FreeImage_Save(FREE_IMAGE_FORMAT_FIF_WEBP, ptr, cname.as_ptr(), 0);
		}
		{
			let filename_out = Path::new("/mnt/c/Developments/photo_png.png");

			let cname = CString::new(filename_out.to_str().unwrap().as_bytes()).unwrap();
			FreeImage_Save(FREE_IMAGE_FORMAT_FIF_PNG, ptr, cname.as_ptr(), 0);
		}
		{
			let filename_out = Path::new("/mnt/c/Developments/photo_tiff.tiff");

			let cname = CString::new(filename_out.to_str().unwrap().as_bytes()).unwrap();
			FreeImage_Save(FREE_IMAGE_FORMAT_FIF_TIFF, ptr, cname.as_ptr(), 0);
		}
		{
			let filename_out = Path::new("/mnt/c/Developments/photo_gif_wu.gif");

			let ptr24 = FreeImage_ConvertTo24Bits(ptr);
			let ptrcolor = FreeImage_ColorQuantize(ptr24, 0);
			let ptr8 = FreeImage_ConvertTo8Bits(ptrcolor);

			let cname = CString::new(filename_out.to_str().unwrap().as_bytes()).unwrap();
			FreeImage_Save(FREE_IMAGE_FORMAT_FIF_GIF, ptr8, cname.as_ptr(), 0);

			FreeImage_Unload(ptr24);
			FreeImage_Unload(ptrcolor);
			FreeImage_Unload(ptr8);
		}
		{
			let filename_out = Path::new("/mnt/c/Developments/photo_gif_neu.gif");

			let ptr24 = FreeImage_ConvertTo24Bits(ptr);
			let ptrcolor = FreeImage_ColorQuantize(ptr24, 1);
			let ptr8 = FreeImage_ConvertTo8Bits(ptrcolor);

			let cname = CString::new(filename_out.to_str().unwrap().as_bytes()).unwrap();
			FreeImage_Save(FREE_IMAGE_FORMAT_FIF_GIF, ptr8, cname.as_ptr(), 0);

			FreeImage_Unload(ptr24);
			FreeImage_Unload(ptrcolor);
			FreeImage_Unload(ptr8);
		}

		FreeImage_Unload(ptr);

		FreeImage_DeInitialise();
	}
}
