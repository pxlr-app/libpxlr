use uuid::Uuid;

pub trait INode {
	fn id(&self) -> Uuid;
	fn display(&self) -> String;
}