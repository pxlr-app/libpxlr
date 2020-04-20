use uuid::Uuid;

pub struct RemoveLayerPatch {
	pub target: Uuid,
	pub child_id: Uuid,
}