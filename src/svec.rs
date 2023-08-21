use core::slice::SliceIndex;


pub struct SVec<T, const N: usize>(Vec<T>);

impl<T, const N: usize> SVec<T, N> {
    pub fn new() -> Self {
        SVec(Vec::with_capacity(N))
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn push(&mut self, item: T) -> Result<(), T> {
        Ok(self.0.push(item))
    }
    pub fn get<I: SliceIndex<[T], Output = T>>(&self, index: I) -> Option<&T> {
        self.0.as_slice().get(index)
    }

    pub fn get_mut<I: SliceIndex<[T], Output = T>>(&mut self, index: I) -> Option<&mut T> {
        self.0.as_mut_slice().get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear(&mut self){
        self.0.clear();
    }
}

impl<T: Clone, const N: usize> SVec<T, N> {
    pub fn resize(&mut self, new_len: usize, value: T) -> Result<(), ()> {
        self.0.resize(new_len, value);
        Ok(())
    }
}

impl<T: Default + Clone, const N: usize> SVec<T, N> {
    pub fn resize_default(&mut self, new_len: usize) -> Result<(), ()> {
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


