use core::slice::SliceIndex;

pub trait UVec<T, const N: usize>:
    UVecNew<T, N>
    + UVecDeref<T, N>
    + UVecGetInd<T, N, usize>
    + UVecResize<T, N>
    + UVecResizeDefault<T, N>
{
}
impl<
        T,
        const N: usize,
        U: UVecNew<T, N>
            + UVecDeref<T, N>
            + UVecGetInd<T, N, usize>
            + UVecResize<T, N>
            + UVecResizeDefault<T, N>,
    > UVec<T, N> for U
{
}

pub trait UVecNew<T, const N: usize> {
    fn new() -> Self;
    fn capacity(&self) -> usize;
    fn push(&mut self, item: T) -> Result<(), T>;
}

pub trait UVecGetInd<T, const N: usize, I> {
    fn get(&self, index: I) -> Option<&T>; //<I: SliceIndex<[T], Output = T>>
    fn get_mut(&mut self, index: I) -> Option<&mut T>; //<I: SliceIndex<[T], Output = T>>
}

pub type Ind<T> = dyn SliceIndex<[T], Output = T>;
pub type UVecGet<T, const N: usize> = dyn UVecGetInd<T, N, usize>;

// pub trait UVecGet<T, const N: usize> {
//     fn get(&self, index: dyn SliceIndex<[T], Output = T>) -> Option<&T>;
//     fn get_mut(&mut self, index: dyn SliceIndex<[T], Output = T>) -> Option<&mut T>;
// }

pub trait UVecResize<T, const N: usize> {
    fn resize(&mut self, new_len: usize, value: T) -> Result<(), ()>;
}

pub trait UVecResizeDefault<T, const N: usize> {
    fn resize_default(&mut self, new_len: usize) -> Result<(), ()>;
}

pub trait UVecDeref<T, const N: usize> {
    fn deref(&self) -> &[T];
    fn deref_mut(&mut self) -> &mut [T];
}

impl<T, const N: usize> std::ops::Deref for dyn UVecDeref<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.deref()
    }
}

impl<T, const N: usize> std::ops::DerefMut for dyn UVecDeref<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.deref_mut()
    }
}

impl<T, const N: usize, Idx> core::ops::Index<Idx> for dyn UVecGetInd<T, N, Idx>
where
    Idx: std::slice::SliceIndex<[T], Output = T>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        self.get(index).unwrap()
    }
}

// pub trait UVecIndex<T, const N: usize, Idx: Sized> {
//     fn get(&self, idx: Idx) -> Option<&T>;
//     fn get_mut(&mut self, idx: Idx) -> Option<&mut T>;
//     // unsafe fn get_unchecked(self, slice: *const Idx) -> *const T;
//     // unsafe fn get_unchecked_mut(self, slice: *mut Idx) -> *mut T;
//     // fn index(self, slice: &Idx) -> &T;
//     // fn index_mut(self, slice: &mut Idx) -> &mut T;
// }

// impl<T, const N: usize, Idx: Sized> std::ops::Index<Idx> for dyn UVecIndex<T, N, Idx> {
//     type Output = T;

//     fn index(&self, index: Idx) -> &Self::Output {
//         self.get(index).unwrap()
//     }
// }

// impl<T, const N: usize, Idx: Sized> std::ops::IndexMut<Idx> for dyn UVecIndex<T, N, Idx> {
//     fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
//         self.get_mut(index).unwrap()
//     }
// }
