#[cfg(feature = "blend_dissolve")]
use crate::rng::Rng;
use crate::{as_rgba::{Float, AsRgba}, blend::{color, color_per_channel}};


#[inline]
pub fn blend_normal<P1: AsRgba, P2: AsRgba>(
    bg: &mut P1,
    fg: &P2,
) {
    if fg.is_fully_transparent() {
        return;
    }

    if bg.is_fully_transparent() || fg.is_fully_opacity() {
        *bg = P1::from_rgba(fg.to_rgba());
        return;
    }

    let [bg_r, bg_g, bg_b, bg_a] = bg.to_rgba();
    let [fg_r, fg_g, fg_b, fg_a] = fg.to_rgba();

    let alpha_final = fg_a + bg_a * (1. - fg_a);
    if alpha_final == 0.0 {
        *bg = P1::EMPTY;
        return;
    };

    let tmp = 1. - fg_a;

    *bg = P1::from_rgba([
        ((fg_r * fg_a) + (bg_r * bg_a) * tmp) / alpha_final,
        ((fg_g * fg_a) + (bg_g * bg_a) * tmp) / alpha_final,
        ((fg_b * fg_a) + (bg_b * bg_a) * tmp) / alpha_final,
        alpha_final
    ])
}

#[cfg(feature = "blend_dissolve")]
#[inline]
pub fn blend_dissolve<P1: AsRgba, P2: AsRgba>(
    bg: &mut P1,
    fg: &P2,
    rng: &mut impl Rng,
) {
    if fg.is_fully_transparent() {
        return;
    }

    if fg.is_fully_opacity() {
        *bg = P1::from_rgba(fg.to_rgba());
        return;
    }
    
    let fg_c = fg.to_rgba();
    if rng.next() <= fg_c[3] {
        *bg = P1::from_rgba([
            fg_c[0],
            fg_c[1],
            fg_c[2],
            1.0,
        ]);
        return;
    }
}

macro_rules! blend {
    ($bg: expr, $fg: expr, ($bg_rgb: pat, $fg_rgb: pat) => $rgb_blend: expr) => {{

        if $fg.is_fully_transparent() {
            return;
        }
        if $bg.is_fully_transparent() {
            *$bg = P1::from_rgba($fg.to_rgba());
            return;
        }
        
        let [bg_r, bg_g, bg_b, bg_a] = $bg.to_rgba();
        let [fg_r, fg_g, fg_b, fg_a] = $fg.to_rgba();
        
        let alpha_final = fg_a + bg_a * (1. - fg_a);
        if alpha_final == 0.0 {
            *$bg = P1::EMPTY;
            return;
        };

        let $bg_rgb = (bg_r, bg_g, bg_b);
        let $fg_rgb = (fg_r, fg_g, fg_b);
        let (out_r, out_g, out_b) = $rgb_blend;
        let (out_r, out_g, out_b) = (
            mix(fg_r, fg_a, bg_r, bg_a, out_r),
            mix(fg_g, fg_a, bg_g, bg_a, out_g),
            mix(fg_b, fg_a, bg_b, bg_a, out_b),
        );
        
        *$bg = P1::from_rgba([
            out_r / alpha_final,
            out_g / alpha_final,
            out_b / alpha_final,
            alpha_final
        ])
    }}
}

#[inline(always)]
fn mix(fg_c: Float, fg_a: Float, bg_c: Float, bg_a: Float, blended_c: Float) -> Float {
    let blend = (1. - bg_a) * fg_c + bg_a * blended_c;
    let cs = blend * fg_a;
    let cb = bg_c * bg_a;
    cs + cb * (1. - fg_a)
}

macro_rules! fn_blend_color_per_channel {
    ($fn_name: ident, $blender: expr) => {
        #[inline]
        pub fn $fn_name<P1: AsRgba, P2: AsRgba>(bg: &mut P1, fg: &P2) {
            blend!(bg, fg, (bg_rgb, fg_rgb) => (
                $blender(bg_rgb.0, fg_rgb.0),
                $blender(bg_rgb.1, fg_rgb.1),
                $blender(bg_rgb.2, fg_rgb.2),
            ))
        }
    }
}

macro_rules! fn_blend_color {
    ($fn_name: ident, $blender: expr) => {
        
        #[inline]
        pub fn $fn_name<P1: AsRgba, P2: AsRgba>(bg: &mut P1, fg: &P2) {
            blend!(bg, fg, (bg_rgb, fg_rgb) => $blender(bg_rgb, fg_rgb))
        }
    }
}

use color_per_channel::*;
use color::*;

fn_blend_color_per_channel!(blend_darken, darken);
fn_blend_color_per_channel!(blend_multiply, multiply);
fn_blend_color_per_channel!(blend_color_burn, color_burn);
fn_blend_color_per_channel!(blend_linear_burn, linear_burn);
fn_blend_color_per_channel!(blend_lighten, lighten);
fn_blend_color_per_channel!(blend_screen, screen);
fn_blend_color_per_channel!(blend_color_dodge, color_dodge);
fn_blend_color_per_channel!(blend_linear_dodge, linear_dodge);
fn_blend_color_per_channel!(blend_overlay, overlay);
fn_blend_color_per_channel!(blend_soft_light, soft_light);
fn_blend_color_per_channel!(blend_hard_light, hard_light);
fn_blend_color_per_channel!(blend_vivid_light, vivid_light);
fn_blend_color_per_channel!(blend_linear_light, linear_light);
fn_blend_color_per_channel!(blend_pin_light, pin_light);
fn_blend_color_per_channel!(blend_hard_mix, hard_mix);
fn_blend_color_per_channel!(blend_difference, difference);
fn_blend_color_per_channel!(blend_exclusion, exclusion);
fn_blend_color_per_channel!(blend_subtract, subtract);
fn_blend_color_per_channel!(blend_divide, divide);

fn_blend_color!(blend_hue, hue);
fn_blend_color!(blend_saturation, saturation);
fn_blend_color!(blend_color, color);
fn_blend_color!(blend_luminosity, luminosity);
fn_blend_color!(blend_darker_color, darker_color);
fn_blend_color!(blend_lighter_color, lighter_color);