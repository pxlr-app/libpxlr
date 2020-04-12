use std::rc::Rc;

use uuid::Uuid;

use crate::group::GroupChild;
use crate::patch::Patch;

pub struct AddChildPatch {
    pub target: Uuid,
    pub child: Rc<dyn GroupChild>,
    pub position: usize,
}

impl Patch for AddChildPatch {
    fn target(&self) -> Uuid {
        self.target
    }
}
