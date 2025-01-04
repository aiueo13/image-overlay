use crate::{blend_mode::BlendMode, rng::FastUnsecurePrng, AsRgba};
use image::{DynamicImage, GenericImage, GenericImageView};


/// Overlay an image at a given coordinate (x, y) with blend mode.  
/// 
/// # Note
/// Do NOT use this function for DynamicImage. Use "overlay_dyn_img" insted.  
/// Because [GenericImage for DynamicImage looses precision](https://github.com/image-rs/image/issues/1592)  and slower.  
/// 
/// # Usage
/// ``````
/// use image::{ImageBuffer, Rgba, Luma};
/// use image_overlay::{overlay, BlendMode};
/// 
/// let mut dest = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(100, 100);
/// let src = ImageBuffer::<Luma<u16>, Vec<u16>>::new(100, 100);
/// 
/// overlay(&mut dest, &src, 0, 0, BlendMode::default());
/// ``````
/// 
/// # Features
/// This crate uses f32 as an intermediate representation by default.  
/// However, can perform higher-precision calculations using f64 by enabling the "f64" feature.  
pub fn overlay<B, F>(bottom: &mut B, top: &F, x: i64, y: i64, blend_mode: BlendMode) 
where 
    B: GenericImage::<Pixel: AsRgba>,
    F: GenericImageView::<Pixel: AsRgba>,
{
    let (
        origin_bottom_x, 
        origin_bottom_y, 
        origin_top_x, 
        origin_top_y, 
        range_width, 
        range_height
    ) = overlay_bounds_ext(bottom.dimensions(), top.dimensions(), x, y);
        
    macro_rules! overlay {
        (($bg: pat, $fg: pat) => $blend: expr) => {{
            for y in 0..range_height {
                for x in 0..range_width {
                    let (bg_x, bg_y) = (origin_bottom_x + x, origin_bottom_y + y);
                    let (fg_x, fg_y) = (origin_top_x + x, origin_top_y + y);

                    #[cfg(debug_assertions)] {
                        let (mut _bg, _fg) = (
                            bottom.get_pixel(bg_x, bg_y),
                            top.get_pixel(fg_x, fg_y),
                        );

                        let ($bg, $fg) = (&mut _bg, &_fg);
                        $blend;

                        bottom.put_pixel(bg_x, bg_y, _bg);
                    }
                    
                    #[cfg(not(debug_assertions))] unsafe {
                        let (mut _bg, _fg) = (
                            bottom.unsafe_get_pixel(bg_x, bg_y),
                            top.unsafe_get_pixel(fg_x, fg_y),
                        );

                        let ($bg, $fg) = (&mut _bg, &_fg);
                        $blend;

                        bottom.unsafe_put_pixel(bg_x, bg_y, _bg);
                    }
                }
            }
        }};
    }

    
    use crate::blend::*;

    match blend_mode {
        BlendMode::Dissolve => {
            // Ensure the same state if the top image size is the same.
            let ref mut rng = FastUnsecurePrng::new(top.width(), top.height());

            overlay!((bg, fg) => blend_dissolve(bg, fg, rng));
        }
        BlendMode::Normal => overlay!((bg, fg) => blend_normal(bg, fg)),
        BlendMode::Darken => overlay!((bg, fg) => blend_darken(bg, fg)),
        BlendMode::Multiply => overlay!((bg, fg) => blend_multiply(bg, fg)),
        BlendMode::ColorBurn => overlay!((bg, fg) => blend_color_burn(bg, fg)),
        BlendMode::LinearBurn => overlay!((bg, fg) => blend_linear_burn(bg, fg)),
        BlendMode::Lighten => overlay!((bg, fg) => blend_lighten(bg, fg)),
        BlendMode::Screen => overlay!((bg, fg) => blend_screen(bg, fg)),
        BlendMode::ColorDodge => overlay!((bg, fg) => blend_color_dodge(bg, fg)),
        BlendMode::LinearDodge => overlay!((bg, fg) => blend_linear_dodge(bg, fg)),
        BlendMode::Overlay => overlay!((bg, fg) => blend_overlay(bg, fg)),
        BlendMode::SoftLight => overlay!((bg, fg) => blend_soft_light(bg, fg)),
        BlendMode::HardLight => overlay!((bg, fg) => blend_hard_light(bg, fg)),
        BlendMode::VividLight => overlay!((bg, fg) => blend_vivid_light(bg, fg)),
        BlendMode::LinearLight => overlay!((bg, fg) => blend_linear_light(bg, fg)),
        BlendMode::PinLight => overlay!((bg, fg) => blend_pin_light(bg, fg)),
        BlendMode::HardMix => overlay!((bg, fg) => blend_hard_mix(bg, fg)),
        BlendMode::Difference => overlay!((bg, fg) => blend_difference(bg, fg)),
        BlendMode::Exclusion => overlay!((bg, fg) => blend_exclusion(bg, fg)),
        BlendMode::Subtract => overlay!((bg, fg) => blend_subtract(bg, fg)),
        BlendMode::Divide => overlay!((bg, fg) => blend_divide(bg, fg)),
        BlendMode::DarkerColor => overlay!((bg, fg) => blend_darker_color(bg, fg)),
        BlendMode::LighterColor => overlay!((bg, fg) => blend_lighter_color(bg, fg)),
        BlendMode::Hue => overlay!((bg, fg) => blend_hue(bg, fg)),
        BlendMode::Saturation => overlay!((bg, fg) => blend_saturation(bg, fg)),
        BlendMode::Color => overlay!((bg, fg) => blend_color(bg, fg)),
        BlendMode::Luminosity => overlay!((bg, fg) => blend_luminosity(bg, fg)),
    }
}

/// Overlay an image at a given coordinate (x, y) with blend mode. 
///  
/// See "overlay" for details.
pub fn overlay_dyn_img(bottom: &mut DynamicImage, top: &DynamicImage, x: i64, y: i64, blend_mode: BlendMode) {
    macro_rules! dynamic_map {
        ($dynimage: expr, $image:pat_param, $action: expr) => {{
            match $dynimage {
                DynamicImage::ImageLuma8($image) => $action,
                DynamicImage::ImageLuma16($image) => $action,
                DynamicImage::ImageLumaA8($image) => $action,
                DynamicImage::ImageLumaA16($image) => $action,
                DynamicImage::ImageRgb8($image) => $action,
                DynamicImage::ImageRgb16($image) => $action,
                DynamicImage::ImageRgb32F($image) => $action,
                DynamicImage::ImageRgba8($image) => $action,
                DynamicImage::ImageRgba16($image) => $action,
                DynamicImage::ImageRgba32F($image) => $action,
                _ => unimplemented!(),
            }
        }};
    }

    dynamic_map!(bottom, bottom, {
        dynamic_map!(top, top, {
            overlay(bottom, top, x, y, blend_mode);
        })
    })
}


/// -------------------------------------------------------
/// THIS FUNCTION IS COPIED FROM image crate (ver. 0.25.5)
/// -------------------------------------------------------
/// 
/// 
/// 
/// Calculate the region that can be copied from top to bottom.
///
/// Given image size of bottom and top image, and a point at which we want to place the top image
/// onto the bottom image, how large can we be? Have to wary of the following issues:
/// * Top might be larger than bottom
/// * Overflows in the computation
/// * Coordinates could be completely out of bounds
///
/// The returned value is of the form:
///
/// `(origin_bottom_x, origin_bottom_y, origin_top_x, origin_top_y, x_range, y_range)`
///
/// The main idea is to do computations on i64's and then clamp to image dimensions.
/// In particular, we want to ensure that all these coordinate accesses are safe:
/// 1. `bottom.get_pixel(origin_bottom_x + [0..x_range), origin_bottom_y + [0..y_range))`
/// 2. `top.get_pixel(origin_top_y + [0..x_range), origin_top_y + [0..y_range))`
fn overlay_bounds_ext(
    (bottom_width, bottom_height): (u32, u32),
    (top_width, top_height): (u32, u32),
    x: i64,
    y: i64,
) -> (u32, u32, u32, u32, u32, u32) {
    // Return a predictable value if the two images don't overlap at all.
    if x > i64::from(bottom_width)
        || y > i64::from(bottom_height)
        || x.saturating_add(i64::from(top_width)) <= 0
        || y.saturating_add(i64::from(top_height)) <= 0
    {
        return (0, 0, 0, 0, 0, 0);
    }

    // Find the maximum x and y coordinates in terms of the bottom image.
    let max_x = x.saturating_add(i64::from(top_width));
    let max_y = y.saturating_add(i64::from(top_height));

    // Clip the origin and maximum coordinates to the bounds of the bottom image.
    // Casting to a u32 is safe because both 0 and `bottom_{width,height}` fit
    // into 32-bits.
    let max_inbounds_x = max_x.clamp(0, i64::from(bottom_width)) as u32;
    let max_inbounds_y = max_y.clamp(0, i64::from(bottom_height)) as u32;
    let origin_bottom_x = x.clamp(0, i64::from(bottom_width)) as u32;
    let origin_bottom_y = y.clamp(0, i64::from(bottom_height)) as u32;

    // The range is the difference between the maximum inbounds coordinates and
    // the clipped origin. Unchecked subtraction is safe here because both are
    // always positive and `max_inbounds_{x,y}` >= `origin_{x,y}` due to
    // `top_{width,height}` being >= 0.
    let x_range = max_inbounds_x - origin_bottom_x;
    let y_range = max_inbounds_y - origin_bottom_y;

    // If x (or y) is negative, then the origin of the top image is shifted by -x (or -y).
    let origin_top_x = x.saturating_mul(-1).clamp(0, i64::from(top_width)) as u32;
    let origin_top_y = y.saturating_mul(-1).clamp(0, i64::from(top_height)) as u32;

    (
        origin_bottom_x,
        origin_bottom_y,
        origin_top_x,
        origin_top_y,
        x_range,
        y_range,
    )
}