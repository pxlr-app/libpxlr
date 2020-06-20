use chrono::{DateTime, Utc};
use document::patch::Patch;
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
	pub redo: Patch,
	pub undo: Patch,
}

pub struct History {
	events: VecDeque<Event>,
	index: usize,
}

impl History {
	pub fn new() -> Self {
		History {
			events: VecDeque::new(),
			index: 0,
		}
	}

	pub fn add(&mut self, event: Event) {
		if self.index < self.events.len() {
			self.forget();
		}
		self.events.push_back(event);
	}

	pub fn forget(&mut self) {
		self.events.drain(self.index..);
	}

	pub fn travel_forward(&mut self) -> Option<&Event> {
		if self.index >= self.events.len() {
			None
		} else {
			let event = self.events.get(self.index).unwrap();
			self.index += 1;
			Some(event)
		}
	}

	pub fn travel_backward(&mut self) -> Option<&Event> {
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
