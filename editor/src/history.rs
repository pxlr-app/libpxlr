use chrono::{DateTime, Utc};
use document::patch::{PatchImpl, Patchable};
use sha2::{Digest, Sha512};
use std::clone::Clone;
use std::collections::VecDeque;
use std::rc::Rc;
use uuid::Uuid;

fn hash_uuid_pair(a: &Uuid, b: &Uuid) -> Uuid {
	let mut hasher = Sha512::new();
	hasher.input(a.as_bytes());
	hasher.input(b.as_bytes());
	let result = hasher.result();
	let mut bytes: [u8; 16] = Default::default();
	bytes.copy_from_slice(&result.as_slice()[0..16]);
	Uuid::from_bytes(bytes)
}

pub struct HistoryAction {
	pub hash: Uuid,
	pub owner: Option<Uuid>,
	pub timestamp: DateTime<Utc>,
	pub undo_patch: Box<dyn PatchImpl>,
	pub redo_patch: Box<dyn PatchImpl>,
}

#[derive(PartialEq, Debug)]
pub struct HistoryState<T>
where
	T: Patchable,
{
	pub hash: Uuid,
	pub data: Rc<T>,
}

impl<T> Clone for HistoryState<T>
where
	T: Patchable,
{
	fn clone(&self) -> HistoryState<T> {
		HistoryState::<T> {
			hash: self.hash,
			data: self.data.clone(),
		}
	}
}

pub struct History<T>
where
	T: Patchable,
{
	pub current_state: HistoryState<T>,
	pub prev_state: HistoryState<T>,
	pub pending_actions: VecDeque<HistoryAction>,
}

impl<T> History<T>
where
	T: Patchable,
{
	fn new(initial_state: HistoryState<T>) -> Self {
		History::<T> {
			prev_state: initial_state.clone(),
			current_state: initial_state,
			pending_actions: VecDeque::new(),
		}
	}

	fn push_action(&mut self, action: HistoryAction) {
		if let Some(new_data) = self.current_state.data.patch(&*action.redo_patch) {
			self.current_state = HistoryState::<T> {
				hash: hash_uuid_pair(&self.current_state.hash, &action.hash),
				data: Rc::new(*new_data),
			};
			self.pending_actions.push_back(action);
		}
	}

	// fn pop_action(&mut self) {
	// 	if let Some(action) = self.pending_actions.pop_back() {
	// 		if let Some(new_data) = self.current_state.data.patch(&*action.undo_patch) {
	// 			self.current_state = HistoryState::<T> {
	// 				hash: hash_uuid_pair(&self.current_state.hash, &action.hash),
	// 				data: Rc::new(*new_data),
	// 			};
	// 		}
	// 	}
	// }

	fn commit_pending_actions(&mut self) {
		let mut action_oredered = self.pending_actions.drain(..).collect::<Vec<_>>();
		action_oredered.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

		for action in action_oredered.into_iter() {
			if let Some(new_data) = self.current_state.data.patch(&*action.redo_patch) {
				self.current_state = HistoryState::<T> {
					hash: hash_uuid_pair(&self.current_state.hash, &action.hash),
					data: Rc::new(*new_data),
				};
			}
		}

		self.prev_state = self.current_state.clone();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::{offset::TimeZone, Utc};
	use document::{patch::Renamable, Note};
	use math::Vec2;
	use std::rc::Rc;
	use uuid::Uuid;

	#[test]
	fn it_basic() {
		let doc = Rc::new(Note::new(None, "A", Vec2::new(0., 0.)));
		let mut history = History::new(HistoryState {
			hash: Uuid::new_v4(),
			data: doc.clone(),
		});

		let (redo, undo) = doc.rename("AA");
		history.push_action(HistoryAction {
			hash: Uuid::new_v4(),
			owner: None,
			timestamp: Utc.ymd(2020, 4, 18).and_hms_milli(9, 37, 0, 10),
			undo_patch: Box::new(undo),
			redo_patch: Box::new(redo),
		});

		assert_eq!(*history.prev_state.data.note, "A");
		assert_eq!(*history.current_state.data.note, "AA");
		assert_eq!(Rc::strong_count(&doc), 2);

		let (redo, undo) = doc.rename("BB");
		history.push_action(HistoryAction {
			hash: Uuid::new_v4(),
			owner: None,
			timestamp: Utc.ymd(2020, 4, 18).and_hms_milli(9, 37, 0, 9),
			undo_patch: Box::new(undo),
			redo_patch: Box::new(redo),
		});

		assert_eq!(*history.prev_state.data.note, "A");
		assert_eq!(*history.current_state.data.note, "BB");
		assert_eq!(Rc::strong_count(&doc), 2);

		history.commit_pending_actions();

		assert_eq!(*history.prev_state.data.note, "AA");
		assert_eq!(*history.current_state.data.note, "AA");
		assert_eq!(Rc::strong_count(&doc), 1);
	}
}
