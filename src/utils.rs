/// A structure to track a count and enforce a limitimum limit.
///
/// The `Limit` struct keeps track of a count that can be incremented or decremented,
/// and it can enforce an optional limitimum limit. If a limitimum limit is set, the struct
/// can check whether the current count exceeds this limit.
///
/// # Fields
///
/// - `count`: The current count, starting at zero.
/// - `limit`: An optional limitimum limit for the count.
///
/// # Examples
///
/// ```
/// let mut limit = Limit::new(Some(5));
/// limit.increment();
/// limit.increment();
/// assert_eq!(limit.get_count(), 2);
/// assert!(!limit.is_over_limit());
/// limit.increment();
/// limit.increment();
/// limit.increment();
/// assert!(limit.is_over_limit());
/// ```
///
/// If the limitimum limit is `None`, the count is considered to have no upper bound.
///
/// ```
/// let mut limit = Limit::new(None);
/// limit.increment();
/// limit.increment();
/// assert_eq!(limit.get_count(), 2);
/// assert!(!limit.is_over_limit());
/// ```
pub struct Limit {
    count: usize,
    limit: Option<usize>,
}

impl Limit {
    pub fn new(limit: Option<usize>) -> Self {
        Self { count: 0, limit }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn decrement(&mut self) {
        self.count -= 1;
    }

    pub fn reset_count(&mut self) {
        self.count = 0;
    }

    pub fn is_under_limit(&self) -> bool {
        if let Some(limit) = self.limit {
            self.count < limit
        } else {
            true
        }
    }

    pub fn is_at_limit(&self) -> bool {
        if let Some(limit) = self.limit {
            self.count + 1 == limit
        } else {
            false
        }
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn get_limit(&self) -> Option<usize> {
        self.limit
    }
}
