use std::fmt;


/// Algorithm for blending pixels.
/// 
/// # References
/// [Adobe Photoshop Blending Modes Documentation](https://helpx.adobe.com/en/photoshop/using/blending-modes.html)  
/// 
/// # Disclaimer
/// * This is NOT a faithful reproduction.
/// * This is NOT affiliated with or endorsed by Adobe Inc.  
/// * Adobe and Photoshop are either registered trademarks or trademarks of Adobe in the United States and/or other countries.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BlendMode {
    #[default]
    Normal,

    #[cfg(feature = "blend_dissolve")]
    Dissolve,
    
    Darken,
    Multiply,
    ColorBurn,
    LinearBurn,
    DarkerColor,
    
    Lighten,
    Screen,
    ColorDodge,
    LinearDodge,
    LighterColor,
    
    Overlay,
    SoftLight,
    HardLight,
    VividLight,
    LinearLight,
    PinLight,
    HardMix,
    
    Difference,
    Exclusion,
    Subtract,
    Divide,
    
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl fmt::Display for BlendMode {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mode_str = match self {
            BlendMode::Normal => "Normal",
            
            #[cfg(feature = "blend_dissolve")]
            BlendMode::Dissolve => "Dissolve",

            BlendMode::Darken => "Darken",
            BlendMode::Multiply => "Multiply",
            BlendMode::ColorBurn => "ColorBurn",
            BlendMode::LinearBurn => "LinearBurn",
            BlendMode::DarkerColor => "DarkerColor",
            BlendMode::Lighten => "Lighten",
            BlendMode::Screen => "Screen",
            BlendMode::ColorDodge => "ColorDodge",
            BlendMode::LinearDodge => "LinearDodge",
            BlendMode::LighterColor => "LighterColor",
            BlendMode::Overlay => "Overlay",
            BlendMode::SoftLight => "SoftLight",
            BlendMode::HardLight => "HardLight",
            BlendMode::VividLight => "VividLight",
            BlendMode::LinearLight => "LinearLight",
            BlendMode::PinLight => "PinLight",
            BlendMode::HardMix => "HardMix",
            BlendMode::Difference => "Difference",
            BlendMode::Exclusion => "Exclusion",
            BlendMode::Subtract => "Subtract",
            BlendMode::Divide => "Divide",
            BlendMode::Hue => "Hue",
            BlendMode::Saturation => "Saturation",
            BlendMode::Color => "Color",
            BlendMode::Luminosity => "Luminosity",
        };
        write!(f, "{}", mode_str)
    }
}