#[cfg(feature="heapless")]
mod hvec;
#[cfg(feature="std")]
mod svec;


#[cfg(feature="heapless")]
pub type Vec<T, const N: usize> = hvec::HVec<T, N>;

#[cfg(feature="std")]
pub type Vec<T, const N: usize> = svec::SVec<T, N>;

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature="heapless")]
    #[test]
    fn it_works() {
        let v: heapless::Vec<u32, 16> = heapless::Vec::new();

        fn check_trait(uvec: &UVecGet<u32, 16>){
            uvec.get(123);
        }

        let re = check_trait(&v);
    }
}
