use std::rc::Rc;
use std::any::Any;

trait Document {
	fn patch(&self, patch: &Patch) -> Option<Self> where Self: Sized;
}

enum Patch<'a> {
	Rename(&'a str),
	Duplicate
}

#[derive(Debug)]
struct Foo {
	pub name: Rc<String>,
}

impl Foo {
	fn new(name: &str) -> Self {
		Foo { name: Rc::new(name.to_owned()) }
	}
}

impl Document for Foo {
	fn patch(&self, patch: &Patch) -> Option<Foo> {
		match &patch {
			Patch::Rename(s) => Some(Foo::new(s)),
			Patch::Duplicate => Some(Foo { name: Rc::clone(&self.name) }),
			_ => None,
		}
	}
}

fn main() {
	let p1 = Patch::Rename("Fuu");
	let p2 = Patch::Duplicate;

	{
		let a = Foo::new("Foo");
		let _a = a.patch(&p1).unwrap();
		assert_eq!(*a.name, "Foo");
		assert_eq!(Rc::strong_count(&a.name), 1);
		assert_eq!(*_a.name, "Fuu");
		assert_eq!(Rc::strong_count(&_a.name), 1);
	}
	{
		let a = Foo::new("Foo");
		let _a = a.patch(&p2).unwrap();
		assert_eq!(*a.name, "Foo");
		assert_eq!(Rc::strong_count(&a.name), 2);
		assert_eq!(*_a.name, "Foo");
		assert_eq!(Rc::strong_count(&_a.name), 2);
	}
	{
		let b: Box<dyn Any> = Box::new(Foo::new("Foo"));
		let a = b.downcast_ref::<Foo>().unwrap();
		let _a = a.patch(&p1).unwrap();
		assert_eq!(*a.name, "Foo");
		assert_eq!(Rc::strong_count(&a.name), 1);
		assert_eq!(*_a.name, "Fuu");
		assert_eq!(Rc::strong_count(&_a.name), 1);
		assert_eq!(b.is::<Foo>(), true);
	}
}
