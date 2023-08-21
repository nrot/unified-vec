#[cfg(feature="heapless")]
mod hvec;
#[cfg(feature="std")]
mod svec;


#[cfg(feature="heapless")]
pub type Vec<T, const N: usize> = hvec::HVec<T, N>;

#[cfg(feature="std")]
pub type Vec<T, const N: usize> = svec::SVec<T, N>;