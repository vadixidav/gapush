use TotalMemory;

use std::vec;
use std::collections::vec_deque::{self, VecDeque};
use std::iter;
use heapsize::HeapSizeOf;

#[derive(Debug, Clone)]
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
            size: 0,
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
    pub fn get(&self, ix: usize) -> Option<&T> {
        self.vec.get(ix)
    }

    #[inline]
    pub fn get_mut(&mut self, ix: usize) -> Option<&mut T> {
        self.vec.get_mut(ix)
    }

    #[inline]
    pub fn into_iter(self) -> TrackedIter<T> {
        TrackedIter {
            iter: self.vec.into_iter(),
            size: self.size,
        }
    }

    #[inline]
    pub fn into_cycle_iter(self) -> TrackedCycleIter<T>
        where T: Clone
    {
        TrackedCycleIter {
            iter: self.vec.into_iter().cycle(),
            size: self.size,
        }
    }
}

impl<T> HeapSizeOf for TrackedVec<T> {
    fn heap_size_of_children(&self) -> usize {
        self.size
    }
}

#[derive(Debug, Clone)]
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

impl<T> HeapSizeOf for TrackedIter<T> {
    fn heap_size_of_children(&self) -> usize {
        self.size
    }
}

#[derive(Debug, Clone)]
pub struct TrackedCycleIter<T> {
    iter: iter::Cycle<vec::IntoIter<T>>,
    size: usize,
}

impl<T> Iterator for TrackedCycleIter<T>
    where T: Clone
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
}

impl<T> HeapSizeOf for TrackedCycleIter<T> {
    fn heap_size_of_children(&self) -> usize {
        self.size
    }
}

#[derive(Debug, Clone)]
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
            size: 0,
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

impl<T> HeapSizeOf for TrackedDeque<T> {
    fn heap_size_of_children(&self) -> usize {
        self.size
    }
}

#[derive(Debug, Clone)]
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

impl<T> HeapSizeOf for TrackedDeqIter<T> {
    fn heap_size_of_children(&self) -> usize {
        self.size
    }
}

