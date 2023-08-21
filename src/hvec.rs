use core::slice::SliceIndex;

use heapless::Vec;

use crate::UVecDeref;

impl<T, const N: usize> super::UVecNew<T, N> for Vec<T, N> {
    fn new() -> Self {
        Vec::new()
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }

    fn push(&mut self, item: T) -> Result<(), T> {
        self.push(item)
    }
}

impl<T: Clone, const N: usize, I> super::UVecGetInd<T, N, I> for Vec<T, N>
where
    I: SliceIndex<[T], Output = T>,
{
    fn get(&self, index: I) -> Option<&T> {
        self.as_slice().get(index)
    }

    fn get_mut(&mut self, index: I) -> Option<&mut T> {
        self.deref_mut().get_mut(index)
    }
}

impl<T: Clone, const N: usize> super::UVecResize<T, N> for Vec<T, N> {
    fn resize(&mut self, new_len: usize, value: T) -> Result<(), ()> {
        self.resize(new_len, value)
    }
}

impl<T: Default + Clone, const N: usize> super::UVecResizeDefault<T, N> for Vec<T, N> {
    fn resize_default(&mut self, new_len: usize) -> Result<(), ()> {
        self.resize_default(new_len)
    }
}

impl<T, const N: usize> super::UVecDeref<T, N> for Vec<T, N> {
    fn deref(&self) -> &[T] {
        self.as_slice()
    }

    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
    }
}