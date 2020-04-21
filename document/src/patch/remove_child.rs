use uuid::Uuid;

pub struct RemoveChildPatch {
	pub target: Uuid,
	pub child_id: Uuid,
}
