use std::rc::Rc;
use std::fmt::{Display, Formatter, Result};

trait Patchable: Display {
	fn patch(&self, rnd: i32) -> Option<Self> where Self: Sized;
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
	fn patch(&self, rnd: i32) -> Option<A> {
		if rnd > 5 {
			Some(A { name: Rc::clone(&self.name) })
		} else {
			None
		}
	}
}

fn main() {
	let str_a = Rc::new("AAA".to_owned());
	println!("str_a count = {}", Rc::strong_count(&str_a));
	
	let a0 = Rc::new(A { name: Rc::clone(&str_a) });
	println!("str_a count = {}", Rc::strong_count(&str_a));

	{
		let a1 = a0.patch(2).and_then(|p| Some(Rc::new(p))).or(Some(Rc::clone(&a0))).unwrap();
		println!("str_a count = {}", Rc::strong_count(&str_a));
	}
	
	{
		let a2 = a0.patch(8).unwrap();
		println!("str_a count = {}", Rc::strong_count(&str_a));
	}

	println!("str_a count = {}", Rc::strong_count(&str_a));
}