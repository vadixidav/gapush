use TotalMemory;

use std::vec;
use std::collections::vec_deque::{self, VecDeque};
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
    pub fn get(&mut self, ix: usize) -> Option<&T> {
        self.vec.get(ix)
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
    iter: vec::IntoIter<T>,
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

pub struct TrackedDeque<T> {
    vec: VecDeque<T>,
    size: usize,
}

impl<T> TrackedDeque<T>
    where T: TotalMemory
{
    #[inline]
    pub fn new() -> TrackedDeque<T> {
        TrackedDeque {
            vec: VecDeque::new(),
            size: mem::size_of::<Self>(),
        }
    }

    #[inline]
    pub fn push(&mut self, e: T) {
        self.size += e.total_memory();
        self.vec.push_back(e);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        let r = self.vec.pop_front();
        if let Some(ref r) = r {
            self.size -= r.total_memory();
        }
        r
    }

    #[inline]
    pub fn get(&mut self, ix: usize) -> Option<&T> {
        self.vec.get(ix)
    }

    #[inline]
    pub fn into_iter(self) -> TrackedDeqIter<T> {
        TrackedDeqIter {
            iter: self.vec.into_iter(),
            size: self.size,
        }
    }
}

impl<T> TotalMemory for TrackedDeque<T> {
    fn total_memory(&self) -> usize {
        self.size
    }
}

pub struct TrackedDeqIter<T> {
    iter: vec_deque::IntoIter<T>,
    size: usize,
}

impl<T> Iterator for TrackedDeqIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
}

impl<T> TotalMemory for TrackedDeqIter<T> {
    fn total_memory(&self) -> usize {
        self.size
    }
}

