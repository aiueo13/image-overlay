// References
// https://github.com/oov/PSDTool/blob/master/src/blend/blend.go
// https://www.w3.org/TR/compositing-1/

use crate::as_rgba::Float;


#[inline(always)]
pub fn hue(cb: (Float, Float, Float), cs: (Float, Float, Float)) -> (Float, Float, Float) {
    // B( Cb, Cs ) = SetLum(SetSat( Cs, Sat( Cb )), Lum( Cb ))
    set_lum(set_sat( cs, sat(cb)), lum(cb))
}

#[inline(always)]
pub fn saturation(cb: (Float, Float, Float), cs: (Float, Float, Float)) -> (Float, Float, Float) {
    // B( Cb, Cs ) = SetLum(SetSat( Cb, Sat( Cs )), Lum( Cb ))
    set_lum(set_sat(cb, sat(cs)), lum(cb))
}

#[inline(always)]
pub fn color(cb: (Float, Float, Float), cs: (Float, Float, Float)) -> (Float, Float, Float) {
    // B( Cb, Cs ) = SetLum( Cs, Lum( Cb ))
    set_lum(cs, lum(cb))
}

#[inline(always)]
pub fn luminosity(cb: (Float, Float, Float), cs: (Float, Float, Float)) -> (Float, Float, Float) {
    // B( Cb, Cs ) = SetLum( Cb, Lum( Cs ))
    set_lum(cb, lum(cs))
}

#[inline(always)]
pub fn darker_color(cb: (Float, Float, Float), cs: (Float, Float, Float)) -> (Float, Float, Float) {
    if lum(cs) < lum(cb) {
        cs
    }
    else {
        cb
    }
}

#[inline(always)]
pub fn lighter_color(cb: (Float, Float, Float), cs: (Float, Float, Float)) -> (Float, Float, Float) {
    if lum(cb) < lum(cs) {
       cs
    }
    else {
        cb
    }
}


#[inline(always)]
fn lum(rgb: (Float, Float, Float)) -> Float {
    (0.3 * rgb.0) + (0.59 * rgb.1) + (0.11 * rgb.2)
}

#[inline(always)]
fn max(rgb: (Float, Float, Float)) -> Float {
    Float::max(Float::max(rgb.0, rgb.1), rgb.2)
}

#[inline(always)]
fn min(rgb: (Float, Float, Float)) -> Float {
    Float::min(Float::min(rgb.0, rgb.1), rgb.2)
}

#[inline(always)]
fn clip_color(rgb: (Float, Float, Float)) -> (Float, Float, Float) {
    let l = lum(rgb);
    let min = min(rgb);
    if min < 0. {
        return (
            l + (rgb.0 - l) * l / (l - min),
            l + (rgb.1 - l) * l / (l - min),
            l + (rgb.2 - l) * l / (l - min),
        );
    }

    let max = max(rgb);
    if max > 1. {
        return (
            l + (rgb.0 - l) * (1. - l) / (max - l),
            l + (rgb.1 - l) * (1. - l) / (max - l),
            l + (rgb.2 - l) * (1. - l) / (max - l),
        );
    }
    rgb
}

#[inline(always)]
fn set_lum(rgb: (Float, Float, Float), l: Float) -> (Float, Float, Float) {
    let d = l - lum(rgb);
    clip_color((rgb.0 + d, rgb.1 + d, rgb.2 + d))
}

#[inline(always)]
fn sat(rgb: (Float, Float, Float)) -> Float {
    max(rgb) - min(rgb)
}

macro_rules! min_mid_max {
    ($rgb: expr, ($min: pat, $mid: pat, $max: pat) => $action: expr) => {
        if $rgb.0 <= $rgb.1 && $rgb.1 <= $rgb.2 {
            let $min = &mut $rgb.0;
            let $mid = &mut $rgb.1;
            let $max = &mut $rgb.2;
            $action
        } 
        else if $rgb.0 <= $rgb.2 && $rgb.2 <= $rgb.1 {
            let $min = &mut $rgb.0;
            let $mid = &mut $rgb.2;
            let $max = &mut $rgb.1;
            $action
        }
        else if $rgb.1 <= $rgb.0 && $rgb.0 <= $rgb.2 {
            let $min = &mut $rgb.1;
            let $mid = &mut $rgb.0;
            let $max = &mut $rgb.2;
            $action
        }
        else if $rgb.1 <= $rgb.2 && $rgb.2 <= $rgb.0 {
            let $min = &mut $rgb.1;
            let $mid = &mut $rgb.2;
            let $max = &mut $rgb.0;
            $action
        }
        else if $rgb.2 <= $rgb.0 && $rgb.0 <= $rgb.1 {
            let $min = &mut $rgb.2;
            let $mid = &mut $rgb.0;
            let $max = &mut $rgb.1;
            $action
        } 
        else {
            let $min = &mut $rgb.2;
            let $mid = &mut $rgb.1;
            let $max = &mut $rgb.0;
            $action
        }
    };
}

#[inline(always)]
fn set_sat(mut rgb: (Float, Float, Float), s: Float) -> (Float, Float, Float) {
    min_mid_max!(rgb, (min, mid, max) => {
        if max > min {
            *mid = ((*mid - *min) / (*max - *min)) * s;
            *max = s;
        }
        else {
            *mid = 0.;
            *max = 0.;
        }
        *min = 0.;
    });

    rgb
}