use std::cmp::{max, min, Ord};
use std::ops::Range;

pub trait Overlaps {
	fn overlaps(&self, other: &Self) -> bool;
}

pub trait Merge {
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
			.filter(|r| r.start != r.end)
			.map(|r| r.clone())
			.collect();
		old_ranges.sort_by(|a, b| a.start.cmp(&b.start));

		let mut remainder =
			old_ranges
				.drain(..)
				.fold::<Option<Range<T>>, _>(None, |merging, range| {
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
				});
		if let Some(last) = remainder.take() {
			merged.push(last);
		}

		merged
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::ops::Range;

	#[test]
	fn it_merge() {
		let ranges: Vec<Range<i32>> = vec![0..1, 1..3, 5..10, 10..20];
		let ranges = ranges.merge_overlapping();
		assert_eq!(ranges, vec![0..3, 5..20]);

		let ranges: Vec<Range<i32>> = vec![0..0, 0..2, 2..2, 0..0, 0..0];
		let ranges = ranges.merge_overlapping();
		assert_eq!(ranges, vec![0..2]);
	}
}
