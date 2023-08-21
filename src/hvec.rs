use core::slice::SliceIndex;

use heapless::Vec;

use crate::UVecDeref;

pub struct HVec<T, const N: usize>(Vec<T, N>);

impl<T, const N: usize> HVec<T, N> {
    fn new() -> Self {
        HVec(Vec::new())
    }

    fn capacity(&self) -> usize {
        self.0.capacity()
    }

    fn push(&mut self, item: T) -> Result<(), T> {
        self.0.push(item)
    }
    fn get<I: SliceIndex<[T], Output = T>>(&self, index: I) -> Option<&T> {
        self.0.as_slice().get(index)
    }

    fn get_mut<I: SliceIndex<[T], Output = T>>(&mut self, index: I) -> Option<&mut T> {
        self.0.deref_mut().get_mut(index)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T: Clone, const N: usize> HVec<T, N> {
    fn resize(&mut self, new_len: usize, value: T) -> Result<(), ()> {
        self.0.resize(new_len, value)
    }
}

impl<T: Default + Clone, const N: usize> HVec<T, N> {
    fn resize_default(&mut self, new_len: usize) -> Result<(), ()> {
        self.0.resize_default(new_len)
    }
}

impl<T, const N: usize> core::ops::Deref for HVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<T, const N: usize> core::ops::DerefMut for HVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.0.as_mut_ptr(), self.len()) }
    }
}

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
