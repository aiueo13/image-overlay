use pcg_mwc::Mwc128XXA32;
use crate::as_rgba::Float;


pub trait Rng {

    /// (0.0 <= value <= 1.0) or (0.0 < value <= 1.0)
    fn next_f32(&mut self) -> f32;

    /// (0.0 <= value <= 1.0) or (0.0 < value <= 1.0)
    fn next_f64(&mut self) -> f64;

    /// (0.0 <= value <= 1.0) or (0.0 < value <= 1.0)
    #[inline(always)]
    fn next(&mut self) -> Float {
        #[cfg(not(feature="f64"))] {
            self.next_f32()
        }

        #[cfg(feature="f64")] {
            self.next_f64()
        }
    }
}


#[derive(Clone, PartialEq, Eq)]
pub struct FastUnsecurePrng(Mwc128XXA32);

impl FastUnsecurePrng {
    
    /// Construct an instance given two keys.
    pub fn new(k1: u32, k2: u32) -> Self {
        Self(Mwc128XXA32::new(k1, k2))
    }
}

impl Rng for FastUnsecurePrng {

    /// 0.0 <= value <= 1.0
    #[inline(always)]
    fn next_f32(&mut self) -> f32 {
        self.0.next() as f32 / u32::MAX as f32
    }

    /// 0.0 <= value <= 1.0
    #[inline(always)]
    fn next_f64(&mut self) -> f64 {
        self.0.next() as f64 / u32::MAX as f64
    }
}