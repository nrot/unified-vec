mod hvec;
mod svec;

mod traits;
pub use traits::*;


#[cfg(test)]
mod tests {
    use std::slice::SliceIndex;

    use super::*;

    #[test]
    fn it_works() {
        let v: heapless::Vec<u32, 16> = heapless::Vec::new();

        fn check_trait(uvec: &UVecGet<u32, 16>){
            uvec.get(123);
        }

        let re = check_trait(&v);
    }
}
