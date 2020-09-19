use chrono::{DateTime, Utc};
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

pub struct Event<T> {
	pub hash: Uuid,
	pub display: String,
	pub timestamp: DateTime<Utc>,
	pub redo: T,
	pub undo: T,
}

pub struct History<T> {
	events: VecDeque<Event<T>>,
	index: usize,
}

impl<T> History<T> {
	pub fn new() -> Self {
		History {
			events: VecDeque::new(),
			index: 0,
		}
	}

	pub fn add(&mut self, event: Event<T>) {
		if self.index < self.events.len() {
			self.forget();
		}
		self.events.push_back(event);
	}

	pub fn forget(&mut self) {
		self.events.drain(self.index..);
	}

	pub fn travel_forward(&mut self) -> Option<&Event<T>> {
		if self.index >= self.events.len() {
			None
		} else {
			let event = self.events.get(self.index).unwrap();
			self.index += 1;
			Some(event)
		}
	}

	pub fn travel_backward(&mut self) -> Option<&Event<T>> {
		if self.index == 0 {
			None
		} else {
			self.index -= 1;
			let event = self.events.get(self.index).unwrap();
			Some(event)
		}
	}

	pub fn into_chronological(self) -> Self {
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
	use chrono::{TimeZone, Utc};
	use document::patch::{IPatchable, Patch, Renamable};
	use document::Note;
	use math::Vec2;
	use std::rc::Rc;
	use uuid::Uuid;

	#[test]
	fn it_moves_forward_and_backward() {
		// Init note with A
		let doc = Rc::new(Note::new(None, "A", Vec2::new(0., 0.)));
		let mut his: History<Patch> = History::new();

		// Push rename to A->B
		let (redo, undo) = doc.rename("B").unwrap();
		his.add(Event {
			hash: Uuid::new_v4(),
			display: "Rename to B".into(),
			timestamp: Utc.ymd(2020, 4, 19).and_hms_milli(2, 0, 0, 0),
			redo: redo,
			undo: undo,
		});

		// Move forward A->B
		let event = his.travel_forward().unwrap();
		let doc_a = doc.patch(&event.redo).unwrap();
		assert_eq!(*doc_a.note, "B");

		// Push rename to B->C
		let (redo, undo) = doc_a.rename("C").unwrap();
		his.add(Event {
			hash: Uuid::new_v4(),
			display: "Rename to C".into(),
			timestamp: Utc.ymd(2020, 4, 19).and_hms_milli(2, 0, 1, 0),
			redo: redo,
			undo: undo,
		});

		// Move forward B->C
		let event = his.travel_forward().unwrap();
		let doc_b = doc_a.patch(&event.redo).unwrap();
		assert_eq!(*doc_b.note, "C");

		// At the end of time
		assert!(his.travel_forward().is_none());

		// Move backward C->B
		let event = his.travel_backward().unwrap();
		let doc_c = doc_b.patch(&event.undo).unwrap();
		assert_eq!(*doc_c.note, "B");

		// Move backward B->A
		let event = his.travel_backward().unwrap();
		let doc_d = doc_c.patch(&event.undo).unwrap();
		assert_eq!(*doc_d.note, "A");

		// At the beginning of time
		assert!(his.travel_backward().is_none());

		// Forget history
		his.forget();

		// No history
		assert!(his.travel_forward().is_none());
		assert!(his.travel_backward().is_none());
	}

	#[test]
	fn it_reorder_events() {
		// Init note with A
		let doc = Rc::new(Note::new(None, "A", Vec2::new(0., 0.)));
		let mut his: History<Patch> = History::new();

		// Push rename A->B at T2
		let (redo, undo) = doc.rename("B").unwrap();
		his.add(Event {
			hash: Uuid::new_v4(),
			display: "Rename to B".into(),
			timestamp: Utc.ymd(2020, 4, 19).and_hms_milli(2, 0, 1, 0),
			redo: redo,
			undo: undo,
		});

		// Move forward A->B
		let event = his.travel_forward().unwrap();
		let doc_a = doc.patch(&event.redo).unwrap();
		assert_eq!(*doc_a.note, "B");

		// Push rename B->C at T1
		let (redo, undo) = doc_a.rename("C").unwrap();
		his.add(Event {
			hash: Uuid::new_v4(),
			display: "Rename to C".into(),
			timestamp: Utc.ymd(2020, 4, 19).and_hms_milli(2, 0, 0, 0),
			redo: redo,
			undo: undo,
		});

		// Move forward B->C
		let event = his.travel_forward().unwrap();
		let doc_b = doc_a.patch(&event.redo).unwrap();
		assert_eq!(*doc_b.note, "C");

		// At the end of time
		assert!(his.travel_forward().is_none());

		// Reorder history (consume old history)
		let mut his = his.into_chronological();

		// Move forward A->C
		let event = his.travel_forward().unwrap();
		let doc_a = doc.patch(&event.redo).unwrap();
		assert_eq!(*doc_a.note, "C");

		// Move forward C->B
		let event = his.travel_forward().unwrap();
		let doc_b = doc_a.patch(&event.redo).unwrap();
		assert_eq!(*doc_b.note, "B");

		// At the end of time
		assert!(his.travel_forward().is_none());
	}
}
