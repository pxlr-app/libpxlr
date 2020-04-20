use uuid::Uuid;

pub struct MoveChildPatch {
	pub target: Uuid,
	pub child_id: Uuid,
	pub position: usize,
}
