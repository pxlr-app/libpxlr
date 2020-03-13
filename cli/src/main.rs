use std::rc::Rc;
use std::fmt::{Display, Formatter, Result};

trait Patchable: Display {
	fn patch(self, rnd: i32) -> Box<dyn Patchable>;
}

pub struct A {
	pub name: Rc<String>,
}

impl Display for A {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "A (name: {})", self.name)
    }
}

impl Patchable for A {
	fn patch(self, rnd: i32) -> Box<dyn Patchable> {
		if rnd > 5 {
			Box::new(A { name: Rc::clone(&self.name) })
		} else {
			Box::new(self)
		}
	}
}

fn main() {
	let str_A = Rc::new("AAA".to_owned());
	println!("str_A count = {}", Rc::strong_count(&str_A));
	let a = A { name: Rc::clone(&str_A) };
	println!("a = {}, a.name count = {}", a, Rc::strong_count(&str_A));
	{
		let a1 = a.patch(2);
		println!("a1 = {}, a.name count = {}", a1, Rc::strong_count(&str_A));
	}
	println!("a.name count = {}", Rc::strong_count(&str_A));
	// let a2 = a.patch(8);
	// println!("a2 = {}, a.name count = {}", a2, Rc::strong_count(&str_A));
}