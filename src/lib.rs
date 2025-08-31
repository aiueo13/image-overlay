//! # image-overlay  
//! This crate supports overlaying images with 27 blend modes.  
//! 
//! # Usage
//! This is to be used with image crate as following:
//!
//! ``````````
//! use image::{DynamicImage, ImageBuffer, Rgba, Luma};
//! use image_overlay::{overlay, overlay_dyn_img, BlendMode};
//! 
//! 
//! let mut dest = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(100, 100);
//! let src = ImageBuffer::<Luma<u16>, Vec<u16>>::new(100, 100);
//! 
//! overlay(&mut dest, &src, 0, 0, BlendMode::default());
//! 
//! 
//! let mut dest = DynamicImage::new_rgba8(100, 100);
//! let src = DynamicImage::new_luma8(100, 100);
//! 
//! // Do NOT use "overlay" for DynamicImage. Use "overlay_dyn_img" insted.  
//! overlay_dyn_img(&mut dest, &src, 0, 0, BlendMode::default());
//! ``````````
//! 
//! # Features
//! Use f32 as an intermediate representation by default.  
//! However, can perform higher-precision calculations using f64 by enabling the "f64" feature.  

mod blend_mode;
mod overlay;
mod as_rgba;

#[cfg(feature = "blend_dissolve")]
pub mod rng;
pub mod blend;

pub use as_rgba::AsRgba;
pub use blend_mode::BlendMode;
pub use overlay::*;