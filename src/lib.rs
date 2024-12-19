//! Do your sorting in linear time.
//!
//! A [xkcd #3026](https://xkcd.com/3026) implementation... [🤡](https://www.explainxkcd.com/wiki/index.php/3026:_Linear_Sort).

use std::sync::OnceLock;
use std::{cmp, thread, time};

const DEFAULT_DURATION: time::Duration = time::Duration::from_secs(1_000_000);

static SLEEP_DURATION: OnceLock<time::Duration> = OnceLock::new();

/// Get the sleep duration used for the linear sorting operation.
///
/// Joke's on you, if the value is not sufficiently large 😜😜.
pub fn linear_sort_sleep_duration() -> time::Duration {
    *SLEEP_DURATION.get_or_init(|| DEFAULT_DURATION)
}

/// Set the sleep duration used for the linear sorting operation, in seconds.
///
/// Returns OK(()) if the duration was set by this call, or Err(time::Duration)
/// if not.
///
/// Joke's on you, if the value is not sufficiently large 😜😜.
pub fn set_linear_sort_sleep_duration(value: time::Duration) -> Result<(), time::Duration> {
    SLEEP_DURATION.set(value)
}

/// Sort a slice in linear time.
///
/// Implements [xkcd #3026](https://xkcd.com/3026) 🤡.
pub trait LinearSort<T> {
    /// Sort slice in linear time.
    ///
    /// See [`sort`].
    ///
    /// [`sort`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort
    fn linear_sort(&mut self);

    /// Sort slice unstable in linear time.
    ///
    /// See [`sort_unstable`].
    ///
    /// [`sort_unstable`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable
    fn linear_sort_unstable(&mut self);

    /// Sort slice in linear time using `compare` function.
    ///
    /// See [`sort_by`].
    ///
    /// [`sort_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by
    fn linear_sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> cmp::Ordering;

    /// Sort slice unstable in linear time using `compare` function.
    ///
    /// See [`sort_unstable_by`].
    ///
    /// [`sort_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by
    fn linear_sort_unstable_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> cmp::Ordering;

    /// Sort slice in linear time using key extraction function `f`.
    ///
    /// See [`sort_by_key`].
    ///
    /// [`sort_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_key
    fn linear_sort_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord;

    /// Sort slice in linear time using key extraction function `f`, caching
    /// the result of `f`, so the key extraction function is run at most once
    /// per element.
    ///
    /// See [`sort_by_cached_key`].
    ///
    /// [`sort_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_cached_key
    fn linear_sort_by_cached_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord;

    /// Sort slice unstable in linear time using key extraction function `f`.
    ///
    /// See [`sort_unstable_by_key`].
    ///
    /// [`sort_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by_key
    fn linear_sort_unstable_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord;
}

impl<T: Ord> LinearSort<T> for [T] {
    fn linear_sort(&mut self) {
        let t = took(|| self.sort());
        hibernate(t, self.len());
    }

    fn linear_sort_unstable(&mut self) {
        let t = took(|| self.sort_unstable());
        hibernate(t, self.len());
    }

    fn linear_sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> cmp::Ordering,
    {
        let t = took(|| self.sort_by(compare));
        hibernate(t, self.len());
    }

    fn linear_sort_unstable_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> cmp::Ordering,
    {
        let t = took(|| self.sort_unstable_by(compare));
        hibernate(t, self.len());
    }

    fn linear_sort_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let t = took(|| self.sort_by_key(f));
        hibernate(t, self.len());
    }

    fn linear_sort_by_cached_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let t = took(|| self.sort_by_cached_key(f));
        hibernate(t, self.len());
    }

    fn linear_sort_unstable_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let t = took(|| self.sort_unstable_by_key(f));
        hibernate(t, self.len());
    }
}

fn took(f: impl FnOnce()) -> time::Duration {
    let start = time::Instant::now();
    f();
    start.elapsed()
}

fn hibernate(elapsed: time::Duration, len: usize) {
    let sleep_duration = nap_for(elapsed, len);
    thread::sleep(sleep_duration)
}

fn nap_for(elapsed: time::Duration, len: usize) -> time::Duration {
    let len = u32::try_from(len).unwrap_or(u32::MAX);
    let zzz = linear_sort_sleep_duration().saturating_mul(len);

    zzz.saturating_sub(elapsed)
}

#[cfg(test)]
mod tests;
