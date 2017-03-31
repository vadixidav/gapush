use TotalMemory;

use std::vec::IntoIter;
use std::mem;

pub struct TrackedVec<T> {
    vec: Vec<T>,
    size: usize,
}

impl<T> TrackedVec<T>
    where T: TotalMemory
{
    #[inline]
    pub fn new() -> TrackedVec<T> {
        TrackedVec {
            vec: Vec::new(),
            size: mem::size_of::<Self>(),
        }
    }

    #[inline]
    pub fn push(&mut self, e: T) {
        self.size += e.total_memory();
        self.vec.push(e);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        let r = self.vec.pop();
        if let Some(ref r) = r {
            self.size -= r.total_memory();
        }
        r
    }

    #[inline]
    pub fn into_iter(self) -> TrackedIter<T> {
        TrackedIter {
            iter: self.vec.into_iter(),
            size: self.size,
        }
    }
}

impl<T> TotalMemory for TrackedVec<T> {
    fn total_memory(&self) -> usize {
        self.size
    }
}

pub struct TrackedIter<T> {
    iter: IntoIter<T>,
    size: usize,
}

impl<T> Iterator for TrackedIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
}

impl<T> TotalMemory for TrackedIter<T> {
    fn total_memory(&self) -> usize {
        self.size
    }
}

