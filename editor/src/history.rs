use chrono::{DateTime, Utc};
use document::patch::PatchImpl;
use sha2::{Digest, Sha512};
use std::collections::VecDeque;
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

pub struct Event {
	pub hash: Uuid,
	pub display: String,
	pub timestamp: DateTime<Utc>,
	pub redo: Box<dyn PatchImpl>,
	pub undo: Box<dyn PatchImpl>,
}

pub struct History {
	events: VecDeque<Event>,
	index: usize,
}

impl History {
	fn new() -> Self {
		History {
			events: VecDeque::new(),
			index: 0,
		}
	}

	fn add(&mut self, event: Event) {
		if self.index < self.events.len() {
			self.forget();
		}
		self.events.push_back(event);
	}

	fn forget(&mut self) {
		self.events.drain(self.index..);
	}

	fn travel_forward(&mut self) -> Option<&Event> {
		if self.index >= self.events.len() {
			None
		} else {
			let event = self.events.get(self.index).unwrap();
			self.index += 1;
			Some(event)
		}
	}

	fn travel_backward(&mut self) -> Option<&Event> {
		if self.index == 0 {
			None
		} else {
			self.index -= 1;
			let event = self.events.get(self.index).unwrap();
			Some(event)
		}
	}

	fn into_chronological(self) -> Self {
		let mut events = self.events.into_iter().collect::<Vec<_>>();
		events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
		History {
			events: events.into(),
			index: 0,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::{offset::TimeZone, Utc};
	use document::{
		patch::{Patchable, Renamable},
		Note,
	};
	use math::Vec2;
	use std::rc::Rc;
	use uuid::Uuid;

	#[test]
	fn it_moves_forward_and_backward() {
		let doc = Rc::new(Note::new(None, "A", Vec2::new(0., 0.)));
		let mut his = History::new();

		let (redo, undo) = doc.rename("B");
		his.add(Event {
			hash: Uuid::new_v4(),
			display: "Rename to B".into(),
			timestamp: Utc.ymd(2020, 4, 19).and_hms_milli(2, 0, 0, 0),
			redo: Box::new(redo),
			undo: Box::new(undo),
		});

		let event = his.travel_forward().unwrap();
		let doc_a = doc.patch(&*event.redo).unwrap();
		assert_eq!(*doc_a.note, "B");

		let (redo, undo) = doc_a.rename("C");
		his.add(Event {
			hash: Uuid::new_v4(),
			display: "Rename to C".into(),
			timestamp: Utc.ymd(2020, 4, 19).and_hms_milli(2, 0, 1, 0),
			redo: Box::new(redo),
			undo: Box::new(undo),
		});

		let event = his.travel_forward().unwrap();
		let doc_b = doc_a.patch(&*event.redo).unwrap();
		assert_eq!(*doc_b.note, "C");

		assert!(his.travel_forward().is_none());

		let event = his.travel_backward().unwrap();
		let doc_c = doc_b.patch(&*event.undo).unwrap();
		assert_eq!(*doc_c.note, "B");

		let event = his.travel_backward().unwrap();
		let doc_d = doc_c.patch(&*event.undo).unwrap();
		assert_eq!(*doc_d.note, "A");

		assert!(his.travel_backward().is_none());

		his.forget();

		assert!(his.travel_forward().is_none());
	}

	#[test]
	fn it_reorder_events() {
		let doc = Rc::new(Note::new(None, "A", Vec2::new(0., 0.)));
		let mut his = History::new();

		let (redo, undo) = doc.rename("B");
		his.add(Event {
			hash: Uuid::new_v4(),
			display: "Rename to B".into(),
			timestamp: Utc.ymd(2020, 4, 19).and_hms_milli(2, 0, 1, 0),
			redo: Box::new(redo),
			undo: Box::new(undo),
		});

		let event = his.travel_forward().unwrap();
		let doc_a = doc.patch(&*event.redo).unwrap();
		assert_eq!(*doc_a.note, "B");

		let (redo, undo) = doc_a.rename("C");
		his.add(Event {
			hash: Uuid::new_v4(),
			display: "Rename to C".into(),
			timestamp: Utc.ymd(2020, 4, 19).and_hms_milli(2, 0, 0, 0),
			redo: Box::new(redo),
			undo: Box::new(undo),
		});

		let event = his.travel_forward().unwrap();
		let doc_b = doc_a.patch(&*event.redo).unwrap();
		assert_eq!(*doc_b.note, "C");

		assert!(his.travel_forward().is_none());

		let mut his = his.into_chronological();

		let event = his.travel_forward().unwrap();
		let doc_a = doc.patch(&*event.redo).unwrap();
		assert_eq!(*doc_a.note, "C");

		let event = his.travel_forward().unwrap();
		let doc_b = doc_a.patch(&*event.redo).unwrap();
		assert_eq!(*doc_b.note, "B");

		assert!(his.travel_forward().is_none());
	}
}
