use image::{Luma, LumaA, Primitive, Rgb, Rgba};


#[cfg(not(feature="f64"))]
pub type Float = f32;

#[cfg(feature="f64")]
pub type Float = f64;

pub trait AsRgba {

    /// With an alpha channel, this is fully transparent.  
    /// Without it, this is fully black.
    const EMPTY: Self;

    /// 0.0 <= value <= 1.0
    fn from_rgba(rgba: [Float; 4]) -> Self;

    /// 0.0 <= value <= 1.0
    fn to_rgba(&self) -> [Float; 4];

    fn is_fully_transparent(&self) -> bool;

    fn is_fully_opacity(&self) -> bool;
}

macro_rules! impl_as_rgba {
    ($T: ty) => {
        impl AsRgba for Rgb<$T> {
            const EMPTY: Self = Rgb([
                <$T>::DEFAULT_MIN_VALUE, 
                <$T>::DEFAULT_MIN_VALUE, 
                <$T>::DEFAULT_MIN_VALUE
            ]);

            #[inline(always)]
            fn is_fully_transparent(&self) -> bool {
                false
            }

            #[inline(always)]
            fn is_fully_opacity(&self) -> bool {
                true
            }

            #[inline(always)]
            fn to_rgba(&self) -> [Float; 4] {
                const MAX: Float = <$T>::DEFAULT_MAX_VALUE as Float;
                [
                    self.0[0] as Float / MAX,
                    self.0[1] as Float / MAX,
                    self.0[2] as Float / MAX,
                    Float::DEFAULT_MAX_VALUE,
                ]
            }

            #[inline(always)]
            fn from_rgba(rgba: [Float; 4]) -> Self {
                const MAX: Float = <$T>::DEFAULT_MAX_VALUE as Float;
                Rgb([
                    (rgba[0] * MAX) as $T,
                    (rgba[1] * MAX) as $T,
                    (rgba[2] * MAX) as $T,
                ])
            }
        } 

        impl AsRgba for Rgba<$T> {
            const EMPTY: Self = Rgba([
                <$T>::DEFAULT_MIN_VALUE, 
                <$T>::DEFAULT_MIN_VALUE, 
                <$T>::DEFAULT_MIN_VALUE,
                <$T>::DEFAULT_MIN_VALUE
            ]);

            #[inline(always)]
            fn is_fully_transparent(&self) -> bool {
                self.0[3] == <$T>::DEFAULT_MIN_VALUE
            }

            #[inline(always)]
            fn is_fully_opacity(&self) -> bool {
                self.0[3] == <$T>::DEFAULT_MAX_VALUE
            }

            #[inline(always)]
            fn to_rgba(&self) -> [Float; 4] {
                const MAX: Float = <$T>::DEFAULT_MAX_VALUE as Float;
                [
                    self.0[0] as Float / MAX,
                    self.0[1] as Float / MAX,
                    self.0[2] as Float / MAX,
                    self.0[3] as Float / MAX,
                ]
            }
        
            #[inline(always)]
            fn from_rgba(rgba: [Float; 4]) -> Self {
                const MAX: Float = <$T>::DEFAULT_MAX_VALUE as Float;
                Rgba([
                    (rgba[0] * MAX) as $T,
                    (rgba[1] * MAX) as $T,
                    (rgba[2] * MAX) as $T,
                    (rgba[3] * MAX) as $T,
                ])
            }
        } 

        impl AsRgba for Luma<$T> {
            const EMPTY: Self = Luma([
                <$T>::DEFAULT_MIN_VALUE, 
            ]);

            #[inline(always)]
            fn is_fully_transparent(&self) -> bool {
                false
            }

            #[inline(always)]
            fn is_fully_opacity(&self) -> bool {
                true
            }

            #[inline(always)]
            fn to_rgba(&self) -> [Float; 4] {
                const MAX: Float = <$T>::DEFAULT_MAX_VALUE as Float;
                let t = self.0[0] as Float / MAX;
                [
                    t,
                    t,
                    t,
                    Float::DEFAULT_MAX_VALUE,
                ]
            }
        
            #[inline(always)]
            fn from_rgba(rgba: [Float; 4]) -> Self {
                const MAX: Float = <$T>::DEFAULT_MAX_VALUE as Float;
                let luma = rgb_to_luma(rgba[0], rgba[1], rgba[2]);
                Luma([(luma * MAX) as $T])
            }
        } 

        impl AsRgba for LumaA<$T> {
            const EMPTY: Self = LumaA([
                <$T>::DEFAULT_MIN_VALUE,
                <$T>::DEFAULT_MIN_VALUE,
            ]);

            #[inline(always)]
            fn is_fully_transparent(&self) -> bool {
                self.0[1] == <$T>::DEFAULT_MIN_VALUE
            }

            #[inline(always)]
            fn is_fully_opacity(&self) -> bool {
                self.0[1] == <$T>::DEFAULT_MAX_VALUE
            }

            #[inline(always)]
            fn to_rgba(&self) -> [Float; 4] {
                const MAX: Float = <$T>::DEFAULT_MAX_VALUE as Float;
                let t = self.0[0] as Float / MAX;
                [
                    t,
                    t,
                    t,
                    self.0[1] as Float / MAX,
                ]
            }

            #[inline(always)]
            fn from_rgba(rgba: [Float; 4]) -> Self {
                const MAX: Float = <$T>::DEFAULT_MAX_VALUE as Float;
                let luma = rgb_to_luma(rgba[0], rgba[1], rgba[2]);
                LumaA([
                    (luma * MAX) as $T,
                    (rgba[3] * MAX) as $T
                ])
            }
        } 
    };
}

impl_as_rgba!(u8);
impl_as_rgba!(u16);
impl_as_rgba!(u32);
impl_as_rgba!(u64);
impl_as_rgba!(usize);

impl_as_rgba!(i8);
impl_as_rgba!(i16);
impl_as_rgba!(i32);
impl_as_rgba!(i64);
impl_as_rgba!(isize);

impl_as_rgba!(f32);
impl_as_rgba!(f64);


#[inline(always)]
fn rgb_to_luma(r: Float, g: Float, b: Float) -> Float {
    r * 0.2126 + g * 0.7152 + b * 0.0722
}