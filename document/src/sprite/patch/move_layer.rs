use uuid::Uuid;

use crate::patch::Patch;

pub struct MoveLayerPatch {
    pub target: Uuid,
    pub child_id: Uuid,
    pub position: usize,
}

impl Patch for MoveLayerPatch {
    fn target(&self) -> Uuid {
        self.target
    }
}
