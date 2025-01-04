// References
// https://github.com/oov/PSDTool/blob/master/src/blend/blend.go
// https://dunnbypaul.net/blends/
// https://www.w3.org/TR/compositing-1/

use crate::as_rgba::Float;


#[inline(always)]
pub fn darken(base: Float, blend: Float) -> Float {
    base.min(blend)
}

#[inline(always)]
pub fn multiply(base: Float, blend: Float) -> Float {
    base * blend
}

#[inline(always)]
pub fn color_burn(base: Float, blend: Float) -> Float {
    if base == 1. {
        1.
    }
    else if blend == 0. {
        0.
    }
    else {
        (1. - (1. - base) / blend).max(0.)
    }
}

#[inline(always)]
pub fn linear_burn(base: Float, blend: Float) -> Float {
    (base + blend - 1.).max(0.)
}

#[inline(always)]
pub fn lighten(base: Float, blend: Float) -> Float {
    base.max(blend)
}

#[inline(always)]
pub fn screen(base: Float, blend: Float) -> Float {
    base + blend - (base * blend)
}

#[inline(always)]
pub fn color_dodge(base: Float, blend: Float) -> Float {
    if base == 0. {
        0.
    }
    else if blend == 1. {
        1.
    }
    else {
        (base / (1. - blend)).min(1.)
    }
}

#[inline(always)]
pub fn linear_dodge(base: Float, blend: Float) -> Float {
    (base + blend).min(1.)
}

#[inline(always)]
pub fn overlay(base: Float, blend: Float) -> Float {
    hard_light(blend, base) // inverted hard_light
}

#[inline(always)]
pub fn soft_light(base: Float, blend: Float) -> Float {
    let d = if base <= 0.25 {
        ((16. * base - 12.) * base + 4.) * base
    }
    else {
        base.sqrt()
    };

    if blend <= 0.5 {
        base - (1. - 2. * blend) * base * (1. - base)
    }
    else {
        base + (2. * blend - 1.) * (d - base)
    }
}

#[inline(always)]
pub fn hard_light(base: Float, blend: Float) -> Float {
    if blend < 0.5 {
        multiply(base, 2. * blend)
    }
    else {
        screen(base, 2. * blend - 1.)
    }
}

#[inline(always)]
pub fn vivid_light(base: Float, blend: Float) -> Float {
    if blend < 0.5 {
        if blend == 0. {
            0.
        }
        else {
            (1. - (1. - base) / (blend * 2.)).max(0.)
        }
    }
    else {
        let tmp = (blend - 0.5) * 2.0;
        if tmp == 1.0 {
            tmp
        }
        else {
            (base / (1.0 - tmp)).min(1.0)
        }
    }
}

#[inline(always)]
pub fn linear_light(base: Float, blend: Float) -> Float {
    if 0.5 < blend {
        (base + (2. * (blend - 0.5))).min(1.)
    }
    else {
        (base + (2.0 * blend) - 1.0).max(0.)
    }
}

#[inline(always)]
pub fn pin_light(base: Float, blend: Float) -> Float {
    if blend < 0.5 {
        let tmp = blend * 2.;
        if tmp < base {
            tmp
        }
        else {
            base
        }
    }
    else {
        let tmp = (blend - 0.5) * 2.0;
        if base < tmp {
            tmp
        }
        else {
            base
        }
    }
}

#[inline(always)]
pub fn hard_mix(base: Float, blend: Float) -> Float {
    let mut tmp;
    if blend < 0.5 {
        tmp = blend * 2.;
        if blend != 0.0 {
            tmp = Float::max(0.0, 1. - (1. - base) / tmp);
        }
    }
    else {
        if base == 0. {
            tmp = 0.;
        }
        else {
            tmp = (blend - 0.5) * 2.0;
            if tmp != 1. {
                tmp = Float::min(1., (base * 1.) / (1. - tmp));
            }
        }
    }

    if tmp < 0.5 {
        0.
    }
    else {
        1.
    }
}

#[inline(always)]
pub fn difference(base: Float, blend: Float) -> Float {
    (base - blend).abs()
}

#[inline(always)]
pub fn exclusion(base: Float, blend: Float) -> Float {
    base + blend - 2. * base * blend
}

#[inline(always)]
pub fn subtract(base: Float, blend: Float) -> Float {
    (base - blend).max(0.)
}

#[inline(always)]
pub fn divide(base: Float, blend: Float) -> Float {
    if blend == 0. {
        1.
    }
    else {
        (base / blend).min(1.)
    }
}