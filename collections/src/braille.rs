pub use bitvec::prelude::*;

pub fn braille_fmt<O: BitOrder, S: BitStore>(bitvec: &BitVec<O, S>) -> String {
	let l = bitvec.len();
	let w = ((l as f32) / 2.).min((l as f32) / 4.).ceil() as usize;
	let h = 1usize;
	braille_fmt2(&bitvec, w, h, "")
}

pub fn braille_fmt2<O: BitOrder, S: BitStore>(
	bitvec: &BitVec<O, S>,
	width: usize,
	height: usize,
	new_line: &str,
) -> String {
	let w = ((width as f32) / 2.).ceil() as usize;
	let h = ((height as f32) / 4.).ceil() as usize;
	let mut grid = vec![vec![0u32; h]; w];
	for i in 0..bitvec.len() {
		if bitvec[i] {
			let x = i % width;
			let y = i / width;
			let ix = ((x as f32) / 2.).floor() as usize;
			let iy = ((y as f32) / 4.).floor() as usize;
			let tx = x % 2;
			let ty = y % 4;
			grid[ix][iy] += if ty >= 3 {
				1 << (ty * 2 + tx)
			} else {
				1 << (tx * 3 + ty)
			};
		}
	}
	let mut out: String = "".into();
	for y in 0..h {
		for x in 0..w {
			out.push(std::char::from_u32(0x2800 + grid[x][y]).unwrap());
		}
		if y + 1 < h {
			out.push_str(new_line);
		}
	}
	out
}
