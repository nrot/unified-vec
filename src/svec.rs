use core::slice::SliceIndex;


pub struct SVec<T, const N: usize>(Vec<T>);

impl<T, const N: usize> SVec<T, N> {
    fn new() -> Self {
        SVec(Vec::with_capacity(N))
    }

    fn capacity(&self) -> usize {
        self.0.capacity()
    }

    fn push(&mut self, item: T) -> Result<(), T> {
        Ok(self.0.push(item))
    }
    fn get<I: SliceIndex<[T], Output = T>>(&self, index: I) -> Option<&T> {
        self.0.as_slice().get(index)
    }

    fn get_mut<I: SliceIndex<[T], Output = T>>(&mut self, index: I) -> Option<&mut T> {
        self.0.as_mut_slice().get_mut(index)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T: Clone, const N: usize> SVec<T, N> {
    fn resize(&mut self, new_len: usize, value: T) -> Result<(), ()> {
        self.0.resize(new_len, value);
        Ok(())
    }
}

impl<T: Default + Clone, const N: usize> SVec<T, N> {
    fn resize_default(&mut self, new_len: usize) -> Result<(), ()> {
        while self.len() < new_len {
            let _ = self.push(T::default());
        }
        Ok(())
    }
}

impl<T, const N: usize> core::ops::Deref for SVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<T, const N: usize> core::ops::DerefMut for SVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.0.as_mut_ptr(), self.len()) }
    }
}



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