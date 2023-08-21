use core::slice::SliceIndex;

impl<T, const N: usize> super::UVecNew<T, N> for Vec<T> {
    fn new() -> Self {
        Vec::with_capacity(N)
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }

    /// Don`t check resize
    fn push(&mut self, item: T) -> Result<(), T> {
        Ok(self.push(item))
    }
}

impl<T, const N: usize, I> super::UVecGetInd<T, N, I> for Vec<T>
where
    I: SliceIndex<[T], Output = T>,
{
    fn get(&self, index: I) -> Option<&T> {
        self.as_slice().get(index)
    }

    fn get_mut(&mut self, index: I) -> Option<&mut T> {
        self.as_mut_slice().get_mut(index)
    }
}

impl<T: Clone, const N: usize> super::UVecResize<T, N> for Vec<T> {
    fn resize(&mut self, new_len: usize, value: T) -> Result<(), ()> {
        Vec::resize(self, new_len, value);
        Ok(())
    }
}

impl<T: Default, const N: usize> super::UVecResizeDefault<T, N> for Vec<T> {
    fn resize_default(&mut self, new_len: usize) -> Result<(), ()> {
        while self.len() < new_len {
            self.push(T::default());
        }
        Ok(())
    }
}

impl<T, const N: usize> super::UVecDeref<T, N> for Vec<T>{
    fn deref(&self) -> &[T] {
        self.as_slice()
    }

    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}