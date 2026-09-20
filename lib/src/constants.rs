/// `NO_INDEX` represents an invalid index.
///
/// [`usize::MAX`] is used as a [sentinel value]. Since a collection's length
/// cannot exceed [`usize::MAX`] and valid indices are always strictly less
/// than its length, [`usize::MAX`] can never be a valid index.
///
/// [sentinel value]: https://en.wikipedia.org/wiki/Sentinel_value
pub(crate) const NO_INDEX: usize = usize::MAX;
