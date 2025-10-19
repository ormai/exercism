/// Yields each item of a and then each item of b
pub fn append<I, J>(mut a: I, mut b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    std::iter::from_fn(move || {
        if let Some(v) = a.next() {
            Some(v)
        } else {
            b.next()
        }
    })
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(mut nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    let mut iter = nested_iter.next();
    std::iter::from_fn(move || {
        loop {
            if let Some(i) = iter.as_mut() {
                if let Some(v) = i.next() {
                    return Some(v);
                } else {
                    iter = nested_iter.next();
                }
            } else {
                return None;
            }
        }
    })
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
#[allow(clippy::manual_find)]
pub fn filter<I, F>(mut iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    std::iter::from_fn(move || {
        for v in iter.by_ref() {
            if predicate(&v) {
                return Some(v);
            }
        }
        None
    })
}

pub fn length<I: Iterator>(iter: I) -> usize {
    let mut len = 0;
    for _ in iter {
        len += 1;
    }
    len
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
#[allow(clippy::manual_map)]
pub fn map<I, F, U>(mut iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    std::iter::from_fn(move || {
        if let Some(v) = iter.next() {
            Some(function(v))
        } else {
            None
        }
    })
}

pub fn foldl<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut acc = initial;
    for v in iter {
        acc = function(acc, v);
    }
    acc
}

pub fn foldr<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut acc = initial;
    while let Some(v) = iter.next_back() {
        acc = function(acc, v);
    }
    acc
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(mut iter: I) -> impl Iterator<Item = I::Item> {
    std::iter::from_fn(move || iter.next_back())
}
