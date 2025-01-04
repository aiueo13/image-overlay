use crate::{as_rgba::{Float, AsRgba}, blend::{color, color_per_channel}, rng::Rng};
use paste::paste;


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
    ($blender: expr) => { paste! {
        
        #[inline]
        pub fn [<blend_ $blender>]<P1: AsRgba, P2: AsRgba>(bg: &mut P1, fg: &P2) {
            blend!(bg, fg, (bg_rgb, fg_rgb) => (
                $blender(bg_rgb.0, fg_rgb.0),
                $blender(bg_rgb.1, fg_rgb.1),
                $blender(bg_rgb.2, fg_rgb.2),
            ))
        }
    }}
}

macro_rules! fn_blend_color {
    ($blender: expr) => { paste! {
        
        #[inline]
        pub fn [<blend_ $blender>]<P1: AsRgba, P2: AsRgba>(bg: &mut P1, fg: &P2) {
            blend!(bg, fg, (bg_rgb, fg_rgb) => $blender(bg_rgb, fg_rgb))
        }
    }}
}

use color_per_channel::*;
use color::*;

fn_blend_color_per_channel!(darken);
fn_blend_color_per_channel!(multiply);
fn_blend_color_per_channel!(color_burn);
fn_blend_color_per_channel!(linear_burn);
fn_blend_color_per_channel!(lighten);
fn_blend_color_per_channel!(screen);
fn_blend_color_per_channel!(color_dodge);
fn_blend_color_per_channel!(linear_dodge);
fn_blend_color_per_channel!(overlay);
fn_blend_color_per_channel!(soft_light);
fn_blend_color_per_channel!(hard_light);
fn_blend_color_per_channel!(vivid_light);
fn_blend_color_per_channel!(linear_light);
fn_blend_color_per_channel!(pin_light);
fn_blend_color_per_channel!(hard_mix);
fn_blend_color_per_channel!(difference);
fn_blend_color_per_channel!(exclusion);
fn_blend_color_per_channel!(subtract);
fn_blend_color_per_channel!(divide);
fn_blend_color!(hue);
fn_blend_color!(saturation);
fn_blend_color!(color);
fn_blend_color!(luminosity);
fn_blend_color!(darker_color);
fn_blend_color!(lighter_color);