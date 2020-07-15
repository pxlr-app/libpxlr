use std::any;

pub trait Any: any::Any {
	fn as_any(&self) -> &dyn any::Any;
	fn as_any_mut(&mut self) -> &mut dyn any::Any;
}

impl<T: any::Any> Any for T {
	fn as_any(&self) -> &dyn any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn any::Any {
		self
	}
}

pub trait Downcast: Any {
	fn is<T: Any>(&self) -> bool {
		self.as_any().is::<T>()
	}
	fn downcast<T: Any>(&self) -> Option<&T> {
		self.as_any().downcast_ref()
	}
	fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
		self.as_any_mut().downcast_mut()
	}
}
