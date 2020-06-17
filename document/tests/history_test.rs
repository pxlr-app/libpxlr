use chrono::{TimeZone, Utc};
use document::patch::{IPatchable, Renamable};
use document::Note;
use document::{Event, History};
use math::Vec2;
use std::rc::Rc;
use uuid::Uuid;

#[test]
fn it_moves_forward_and_backward() {
	// Init note with A
	let doc = Rc::new(Note::new(None, "A", Vec2::new(0., 0.)));
	let mut his = History::new();

	// Push rename to A->B
	let (redo, undo) = doc.rename("B").unwrap();
	his.add(Event {
		hash: Uuid::new_v4(),
		display: "Rename to B".into(),
		timestamp: Utc.ymd(2020, 4, 19).and_hms_milli(2, 0, 0, 0),
		redo: Box::new(redo),
		undo: Box::new(undo),
	});

	// Move forward A->B
	let event = his.travel_forward().unwrap();
	let doc_a = doc.patch(&*event.redo).unwrap();
	assert_eq!(*doc_a.note, "B");

	// Push rename to B->C
	let (redo, undo) = doc_a.rename("C").unwrap();
	his.add(Event {
		hash: Uuid::new_v4(),
		display: "Rename to C".into(),
		timestamp: Utc.ymd(2020, 4, 19).and_hms_milli(2, 0, 1, 0),
		redo: Box::new(redo),
		undo: Box::new(undo),
	});

	// Move forward B->C
	let event = his.travel_forward().unwrap();
	let doc_b = doc_a.patch(&*event.redo).unwrap();
	assert_eq!(*doc_b.note, "C");

	// At the end of time
	assert!(his.travel_forward().is_none());

	// Move backward C->B
	let event = his.travel_backward().unwrap();
	let doc_c = doc_b.patch(&*event.undo).unwrap();
	assert_eq!(*doc_c.note, "B");

	// Move backward B->A
	let event = his.travel_backward().unwrap();
	let doc_d = doc_c.patch(&*event.undo).unwrap();
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
	let mut his = History::new();

	// Push rename A->B at T2
	let (redo, undo) = doc.rename("B").unwrap();
	his.add(Event {
		hash: Uuid::new_v4(),
		display: "Rename to B".into(),
		timestamp: Utc.ymd(2020, 4, 19).and_hms_milli(2, 0, 1, 0),
		redo: Box::new(redo),
		undo: Box::new(undo),
	});

	// Move forward A->B
	let event = his.travel_forward().unwrap();
	let doc_a = doc.patch(&*event.redo).unwrap();
	assert_eq!(*doc_a.note, "B");

	// Push rename B->C at T1
	let (redo, undo) = doc_a.rename("C").unwrap();
	his.add(Event {
		hash: Uuid::new_v4(),
		display: "Rename to C".into(),
		timestamp: Utc.ymd(2020, 4, 19).and_hms_milli(2, 0, 0, 0),
		redo: Box::new(redo),
		undo: Box::new(undo),
	});

	// Move forward B->C
	let event = his.travel_forward().unwrap();
	let doc_b = doc_a.patch(&*event.redo).unwrap();
	assert_eq!(*doc_b.note, "C");

	// At the end of time
	assert!(his.travel_forward().is_none());

	// Reorder history (consume old history)
	let mut his = his.into_chronological();

	// Move forward A->C
	let event = his.travel_forward().unwrap();
	let doc_a = doc.patch(&*event.redo).unwrap();
	assert_eq!(*doc_a.note, "C");

	// Move forward C->B
	let event = his.travel_forward().unwrap();
	let doc_b = doc_a.patch(&*event.redo).unwrap();
	assert_eq!(*doc_b.note, "B");

	// At the end of time
	assert!(his.travel_forward().is_none());
}
