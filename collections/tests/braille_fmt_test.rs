use collections::{bitvec, braille_fmt, braille_fmt2};

#[test]
fn it_braille_fmt() {
	let v = bitvec![1, 0, 1, 1, 1];
	assert_eq!(braille_fmt(&v), "⠗");
	let v = bitvec![1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1];
	assert_eq!(braille_fmt(&v), "⢕⡝");
}

#[test]
fn it_braille_fmt2() {
	let v = bitvec![1, 1, 1];
	assert_eq!(braille_fmt2(&v, 1, 3, ""), "⠇");
	let v = bitvec![1, 1, 1];
	assert_eq!(braille_fmt2(&v, 3, 1, ""), "⠉⠁");
	let v = bitvec![1, 0, 1, 1, 1];
	assert_eq!(braille_fmt2(&v, 5, 1, ""), "⠁⠉⠁");
	let v = bitvec![1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1];
	assert_eq!(braille_fmt2(&v, 15, 1, ""), "⠁⠉⠈⠈⠁⠁⠈⠁");
}
