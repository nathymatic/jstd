/// * Returns a new vector containing only the elements that satisfy `predicate`.
pub fn filter<T, F>(items: &[T], predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
    T: Clone,
{
    items.iter().filter(|i| predicate(i)).cloned().collect()
}

/// * Transforms every element into another value.
pub fn map<T, U, F>(items: &[T], transform: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    items.iter().map(transform).collect()
}

/// * Returns the first element satisfying `predicate`.
pub fn find<T, F>(items: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    items.iter().find(|i| predicate(i))
}

/// * Returns whether any element satisfies `predicate`.
pub fn any<T, F>(items: &[T], predicate: F) -> bool
where
    F: Fn(&T) -> bool,
{
    items.iter().any(predicate)
}

/// * Returns whether every element satisfies `predicate`.
pub fn all<T, F>(items: &[T], predicate: F) -> bool
where
    F: Fn(&T) -> bool,
{
    items.iter().all(predicate)
}

/// * Returns the number of elements satisfying `predicate`.
pub fn count<T, F>(items: &[T], predicate: F) -> usize
where
    F: Fn(&T) -> bool,
{
    items.iter().filter(|i| predicate(i)).count()
}

/// * Returns a new vector with duplicate elements removed.
///
/// * Requires `T` to implement `PartialEq`
pub fn unique<T>(items: &[T]) -> Vec<T>
where
    T: Clone + PartialEq,
{
    let mut result = Vec::new();

    for item in items {
        if !result.contains(item) {
            result.push(item.clone());
        }
    }

    result
}

/// * Splits a slice into chunks of at most `size` elements.
pub fn chunk<T: Clone>(items: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 0 {
        return Vec::new();
    }

    items.chunks(size).map(|c| c.to_vec()).collect()
}

/// * Splits elements into matching and non-matching groups.
pub fn partition<T, F>(items: &[T], predicate: F) -> (Vec<T>, Vec<T>)
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    let mut matching = Vec::new();
    let mut non_matching = Vec::new();

    for item in items {
        if predicate(item) {
            matching.push(item.clone());
        } else {
            non_matching.push(item.clone());
        }
    }

    (matching, non_matching)
}
