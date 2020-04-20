use uuid::Uuid;

pub struct MoveLayerPatch {
	pub target: Uuid,
	pub child_id: Uuid,
	pub position: usize,
}