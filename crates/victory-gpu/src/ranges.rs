//! Ranges cut from a big block and given back: the bookkeeping behind
//! memory the host is asked for in large pieces instead of once per
//! texture (staging memory in [`crate::texture`], texture memory in
//! [`crate::resource`]).
//!
//! Why: in the opening cutscene the pass over textures picked by index
//! spent 80-95% of its 1.5 s a frame asking the host for memory, one
//! committed resource per staging buffer (about 0.5 ms each, 1,000-2,200 a
//! frame) and one per host texture (run `dl_diag1`). A range cut from a
//! block that is there already costs a map lookup.

use std::collections::BTreeMap;

/// The free ranges of one block, by offset.
pub(crate) struct Ranges {
    free: BTreeMap<u64, u64>,
    /// Free bytes in all.
    room: u64,
}

impl Ranges {
    pub(crate) fn new(size: u64) -> Ranges {
        Ranges { free: BTreeMap::from([(0, size)]), room: size }
    }

    /// Free bytes in all (an upper bound for what [`take`](Self::take)
    /// can give).
    #[cfg(test)]
    pub(crate) fn room(&self) -> u64 {
        self.room
    }

    /// The offset of `len` bytes at a multiple of `align` (a power of
    /// two), the first that fits; `None` if nothing does.
    pub(crate) fn take(&mut self, len: u64, align: u64) -> Option<u64> {
        if len == 0 || len > self.room {
            return None;
        }
        let (&at, &size) = self.free.iter().find(|&(&at, &size)| {
            let start = at.next_multiple_of(align);
            start.checked_add(len).is_some_and(|end| end <= at + size)
        })?;
        let start = at.next_multiple_of(align);
        self.free.remove(&at);
        if start > at {
            self.free.insert(at, start - at);
        }
        if start + len < at + size {
            self.free.insert(start + len, at + size - (start + len));
        }
        self.room -= len;
        Some(start)
    }

    /// Gives back `len` bytes at `offset` that [`take`](Self::take) gave,
    /// joined with free neighbours.
    pub(crate) fn give(&mut self, offset: u64, len: u64) {
        let (mut at, mut size) = (offset, len);
        if let Some((&before, &its)) = self.free.range(..offset).next_back()
            && before + its == offset
        {
            self.free.remove(&before);
            (at, size) = (before, its + size);
        }
        if let Some(&after) = self.free.get(&(offset + len)) {
            self.free.remove(&(offset + len));
            size += after;
        }
        self.free.insert(at, size);
        self.room += len;
    }

    #[cfg(test)]
    fn pieces(&self) -> Vec<(u64, u64)> {
        self.free.iter().map(|(&a, &s)| (a, s)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Ranges;

    #[test]
    fn takes_first_fit_and_joins_what_is_given_back() {
        let mut r = Ranges::new(1 << 20);
        let a = r.take(4096, 4096).unwrap();
        let b = r.take(8192, 4096).unwrap();
        let c = r.take(4096, 4096).unwrap();
        assert_eq!((a, b, c), (0, 4096, 12288));
        assert_eq!(r.room(), (1 << 20) - 16384);
        // The middle one back: a hole of its size, reused by the next ask
        // that fits and not by a bigger one.
        r.give(b, 8192);
        assert_eq!(r.pieces(), vec![(4096, 8192), (16384, (1 << 20) - 16384)]);
        assert_eq!(r.take(12288, 4096), Some(16384));
        assert_eq!(r.take(8192, 4096), Some(4096));
        // Everything back, in an order that joins before and after.
        r.give(16384, 12288);
        r.give(a, 4096);
        r.give(c, 4096);
        r.give(4096, 8192);
        assert_eq!(r.pieces(), vec![(0, 1 << 20)]);
        assert_eq!(r.room(), 1 << 20);
    }

    #[test]
    fn keeps_the_alignment_and_the_bytes_before_it() {
        let mut r = Ranges::new(1 << 20);
        assert_eq!(r.take(4096, 4096), Some(0));
        // 64 KiB aligned: the 60 KiB before it stay free.
        assert_eq!(r.take(65536, 65536), Some(65536));
        assert_eq!(r.pieces()[0], (4096, 61440));
        assert_eq!(r.take(61440, 4096), Some(4096));
        assert_eq!(r.pieces(), vec![(131072, (1 << 20) - 131072)]);
    }

    #[test]
    fn refuses_what_does_not_fit() {
        let mut r = Ranges::new(65536);
        assert_eq!(r.take(0, 4096), None);
        assert_eq!(r.take(65537, 4096), None);
        assert_eq!(r.take(32768, 4096), Some(0));
        // 32 KiB are free, but not at a multiple of 64 KiB.
        assert_eq!(r.take(32768, 65536), None);
        assert_eq!(r.take(32768, 4096), Some(32768));
        assert_eq!(r.take(4096, 4096), None);
        assert_eq!(r.room(), 0);
    }
}
