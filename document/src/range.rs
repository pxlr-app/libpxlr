use std::cmp::{max, min, Ord};
use std::ops::Range;

pub(crate) trait Overlaps {
	fn overlaps(&self, other: &Self) -> bool;
}

pub(crate) trait Merge {
	fn merge(&mut self, other: &Self);
}

impl<T> Overlaps for Range<T>
where
	T: PartialOrd,
{
	fn overlaps(&self, other: &Self) -> bool {
		(self.start <= other.start && other.start <= self.end)
			|| (self.start <= other.end && other.end <= self.end)
	}
}

impl<T> Merge for Range<T>
where
	T: Ord + Copy,
{
	fn merge(&mut self, other: &Self) {
		self.start = min(self.start, other.start);
		self.end = max(self.end, other.end);
	}
}

pub(crate) fn merge_ranges<T>(ranges: &mut Vec<Range<T>>)
where
	T: Ord + Copy,
{
	let mut old_ranges: Vec<Range<T>> = ranges.drain(..).collect();
	old_ranges.sort_by(|a, b| a.start.cmp(&b.start));

	let mut remainder = old_ranges
		.drain(..)
		.fold::<Option<Range<T>>, _>(None, |merging, range| {
			if let Some(mut last) = merging {
				if last.overlaps(&range) {
					last.merge(&range);
					return Some(last);
				} else {
					ranges.push(last);
					return Some(range);
				}
			} else {
				return Some(range);
			}
		});
	if let Some(last) = remainder.take() {
		ranges.push(last);
	}
}
