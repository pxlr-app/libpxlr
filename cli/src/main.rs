struct Foo {
	pub bar: i32,
}

impl Foo {
	fn baz(self) -> i32 {
		1
	}
	fn baz_ref(&self) -> i32 {
		1
	}
}

fn main() {
	let foo = Foo { bar: 10 };
	println!("Foo.bar: {}", foo.bar);
	// let baz = foo.baz();
	// println!("Foo.baz: {}", baz);
	// println!("Foo.bar: {}", foo.bar); // baz() consumed foo

	let baz = foo.baz_ref();
	println!("Foo.baz: {}", baz);
	println!("Foo.bar: {}", foo.bar);
}