use core::slice::SliceIndex;

use heapless::Vec;

use crate::UVecDeref;

pub struct HVec<T, const N: usize>(Vec<T, N>);

impl<T, const N: usize> HVec<T, N> {
    pub fn new() -> Self {
        HVec(Vec::new())
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn push(&mut self, item: T) -> Result<(), T> {
        self.0.push(item)
    }
    pub fn get<I: SliceIndex<[T], Output = T>>(&self, index: I) -> Option<&T> {
        self.0.as_slice().get(index)
    }

    pub fn get_mut<I: SliceIndex<[T], Output = T>>(&mut self, index: I) -> Option<&mut T> {
        self.0.deref_mut().get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear(&mut self){
        self.0.clear();
    }
}

impl<T: Clone, const N: usize> HVec<T, N> {
    pub fn resize(&mut self, new_len: usize, value: T) -> Result<(), ()> {
        self.0.resize(new_len, value)
    }
}

impl<T: Default + Clone, const N: usize> HVec<T, N> {
    pub fn resize_default(&mut self, new_len: usize) -> Result<(), ()> {
        self.0.resize_default(new_len)
    }
}

impl<T: Clone, const N:usize> Clone for HVec<T, N>{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T:Default, const N:usize> Default for HVec<T, N>{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a HVec<T, N> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut HVec<T, N> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}