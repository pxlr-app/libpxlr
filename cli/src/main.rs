use std::rc::Rc;
use std::any::Any;

// https://www.reddit.com/r/rust/comments/btloop/dyn_trait_and_boxdyn_trait/

trait Node {
	fn id(&self) -> u32;
}

trait PatchData {
	fn target(&self) -> u32;
}

trait Patch: PatchData {
	fn as_any(&self) -> &dyn Any;
}

impl<T> Patch for T
where
	T: PatchData + Any,
{
	fn as_any(&self) -> &dyn Any {
		self
	}
}

trait Patcher {
	fn patch(&self, patch: &dyn Patch) -> Option<Box<Self>>;
}

trait Patchable: Node {
	fn patch_box(&self, patch: &dyn Patch) -> Option<Box<dyn Patchable + 'static>>;
	fn patch_rc(&self, patch: &dyn Patch) -> Option<Rc<dyn Patchable + 'static>>;
	fn as_any(&self) -> &dyn Any;
}

impl<T> Patchable for T
where
	T: Patcher + Node + Any,
{
	fn patch_box(&self, patch: &dyn Patch) -> Option<Box<dyn Patchable + 'static>> {
		match self.patch(patch) {
			Some(new_self) => Some(new_self),
			None => None
		}
	}
	fn patch_rc(&self, patch: &dyn Patch) -> Option<Rc<dyn Patchable + 'static>> {
		match self.patch(patch) {
			Some(new_self) => Some(Rc::new(*new_self)),
			None => None
		}
	}
	fn as_any(&self) -> &dyn Any {
		self
	}
}

struct Label {
	pub name: Rc<String>,
}

impl std::fmt::Display for Label {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Label(name={})", self.name)
	}
}

impl Node for Label {
	fn id(&self) -> u32 {
		0
	}
}

impl Patcher for Label {
	fn patch(&self, patch: &dyn Patch) -> Option<Box<Label>> {
		if patch.target() == self.id() {
			let patch_any = patch.as_any();
			if let Some(rename) = patch_any.downcast_ref::<RenamePatch>() {
				Some(Box::new(Label { name: Rc::new(rename.name.to_string()) }))
			}
			else
			{
				None
			}
		}
		else
		{
			None
		}
	}
}

struct Group {
	pub children: Rc<Vec<Rc<dyn Patchable>>>,
}

impl std::fmt::Display for Group {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Group(children=...)")
	}
}

impl Node for Group {
	fn id(&self) -> u32 {
		1
	}
}

impl Patcher for Group {
	fn patch(&self, patch: &dyn Patch) -> Option<Box<Group>> {
		// let children = self.children.iter().map(|child| child.patch_rc(patch)).collect::<Vec<_>>();
		// Group { children: Rc::new(children) }
		None
	}
}

struct RenamePatch<'a> {
	pub id: u32,
	pub name: &'a str,
}

impl<'a> PatchData for RenamePatch<'a> {
	fn target(&self) -> u32 {
		self.id
	}
}

fn main() {
	println!("image-rs : https://github.com/image-rs/image#61-opening-and-saving-images");

	
	{
		let r1 = RenamePatch { id: 0, name: "Baz" };

		let s1 = Rc::new("Foo".to_owned());
		let l1 = Label { name: s1.clone() };
		println!("{}", l1);
		assert_eq!(Rc::strong_count(&s1), 2);
		let d1: &dyn Patchable = &l1;
		let p1 = d1.patch_box(&r1).unwrap();
		assert_eq!(Rc::strong_count(&s1), 2);
		let l1a = p1.as_any().downcast_ref::<Label>().unwrap();
		println!("{}", l1a);

		let s2 = Rc::new("Foo".to_owned());
		let l2 = Box::new(Label { name: s2.clone() });
		assert_eq!(Rc::strong_count(&s2), 2);
		let _b2 = l2.patch_box(&r1);
		assert_eq!(Rc::strong_count(&s2), 2);
		let _l2a = l2.patch(&r1);
		assert_eq!(Rc::strong_count(&s2), 2);

		let s3 = Rc::new("Foo".to_owned());
		let v3: Vec<Box<dyn Patchable>> = vec![Box::new(Label { name: s3.clone() })];
		assert_eq!(Rc::strong_count(&s3), 2);
		let _b3 = v3.get(0).unwrap();
		let p3 = _b3.patch_box(&r1).unwrap();
		assert_eq!(Rc::strong_count(&s3), 2);
		let l3a = p3.as_any().downcast_ref::<Label>().unwrap();
		println!("{}", l3a);
	}

	// {
	// 	let s1 = "L1".to_owned();
	// 	let l1 = Label { name: Rc::new(s1.clone()) };
	// 	let g1 = Group { children: Rc::new(vec![Rc::new(l1)]) };
	// 	let _g2 = g1.patch(&r1);
	// 	let b1: Box<dyn Patchable> = Box::new(g1);
	// 	let b2 = b1.patch_box(&r1).unwrap();
	// 	let g3 = b2.as_any().downcast_ref::<Group>().unwrap();
	// 	let l2 = g3.children.get(0).unwrap().as_any().downcast_ref::<Label>().unwrap();
	// 	println!("{}", l2);
	// }
}