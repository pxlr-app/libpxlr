pub use bitvec::prelude::*;

pub fn braille_fmt(bitvec: &BitVec) -> String {
	let l = bitvec.len();
	let w = ((l as f32) / 2.).min((l as f32) / 4.).ceil() as usize;
	let w = w / 2;
	let h = 1usize;
	braille_fmt2(&bitvec, w, h, "")
}

pub fn braille_fmt2(bitvec: &BitVec, width: usize, height: usize, new_line: &str) -> String {
	let translate: Vec<Vec<u32>> = vec![vec![1, 2, 4, 64], vec![8, 16, 32, 128]];
	let mut grid = vec![vec![0u32; height]; width];
	for i in 0..bitvec.len() {
		let x = i % (width * 2);
		let y = i / (width * 2);
		let ix = ((x as f32) / 2.).floor() as usize;
		let iy = ((y as f32) / 4.).floor() as usize;
		let tx = x % 2;
		let ty = y % 4;
		if bitvec[i] {
			grid[ix][iy] += translate[tx][ty];
		}
	}
	let mut out: String = "".into();
	for y in 0..height {
		for x in 0..width {
			out.push(std::char::from_u32(0x2800 + grid[x][y]).unwrap());
		}
		if y + 1 < height {
			out.push_str(new_line);
		}
	}
	out
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_braille_fmt() {
		let v = bitvec![1, 0, 1, 1, 1];
		assert_eq!(braille_fmt(&v), "⠗");
		let v = bitvec![1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1];
		assert_eq!(braille_fmt(&v), "⢕⡝");
	}

	#[test]
	fn it_braille_fmt2() {
		let v = bitvec![1, 0, 1, 1, 1];
		assert_eq!(braille_fmt2(&v, 3, 1, ""), "⠁⠉⠁");
		let v = bitvec![1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1];
		assert_eq!(braille_fmt2(&v, 4, 1, ""), "⠃⠋⠘⠊");
	}
}
