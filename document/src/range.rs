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

pub trait MergeOverlapping {
	fn merge_overlapping(&self) -> Self;
}

impl<T> MergeOverlapping for Vec<Range<T>>
where
	T: Ord + Copy,
{
	fn merge_overlapping(&self) -> Self {
		let mut merged: Vec<Range<T>> = Vec::with_capacity(self.len());
		let mut old_ranges: Vec<Range<T>> = self
			.iter()
			.map(|r| Range {
				start: r.start,
				end: r.end,
			})
			.collect();
		old_ranges.sort_by(|a, b| a.start.cmp(&b.start));

		let mut remainder =
			old_ranges
				.drain(..)
				.fold::<Option<Range<T>>, _>(None, |merging, range| {
					if range.start != range.end {
						if let Some(mut last) = merging {
							if last.overlaps(&range) {
								last.merge(&range);
								return Some(last);
							} else {
								merged.push(last);
								return Some(range);
							}
						} else {
							return Some(range);
						}
					} else {
						return None;
					}
				});
		if let Some(last) = remainder.take() {
			merged.push(last);
		}

		merged
	}
}
