use uuid::Uuid;

pub trait Node {
	fn id(&self) -> Uuid;
}
