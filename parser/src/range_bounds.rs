use std::ops::{Bound, Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

pub trait RangeBounds<T> {
	fn start_bound(&self) -> Bound<&T>;
	fn end_bound(&self) -> Bound<&T>;
}

impl<T> RangeBounds<T> for Range<T> {
	fn start_bound(&self) -> Bound<&T> {
		Bound::Included(&self.start)
	}
	fn end_bound(&self) -> Bound<&T> {
		Bound::Excluded(&self.end)
	}
}

impl<T> RangeBounds<T> for RangeFrom<T> {
	fn start_bound(&self) -> Bound<&T> {
		Bound::Included(&self.start)
	}
	fn end_bound(&self) -> Bound<&T> {
		Bound::Unbounded
	}
}

impl<T> RangeBounds<T> for RangeTo<T> {
	fn start_bound(&self) -> Bound<&T> {
		Bound::Unbounded
	}
	fn end_bound(&self) -> Bound<&T> {
		Bound::Excluded(&self.end)
	}
}

impl<T> RangeBounds<T> for RangeInclusive<T> {
	fn start_bound(&self) -> Bound<&T> {
		Bound::Included(&self.start())
	}
	fn end_bound(&self) -> Bound<&T> {
		Bound::Excluded(&self.end())
	}
}

impl<T> RangeBounds<T> for RangeToInclusive<T> {
	fn start_bound(&self) -> Bound<&T> {
		Bound::Unbounded
	}
	fn end_bound(&self) -> Bound<&T> {
		Bound::Included(&self.end)
	}
}
