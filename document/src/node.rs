use std::any::Any;

use uuid::Uuid;

pub trait Node {
    fn id(&self) -> Uuid;
}

pub trait NodeImpl: Node {
    fn as_any(&self) -> &dyn Any;
}

impl<T> NodeImpl for T
where
    T: Node + Any,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}
