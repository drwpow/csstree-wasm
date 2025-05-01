pub mod units {

    pub const LENGTH: &[&str] = &[
        // absolute length units: https://www.w3.org/TR/css-values-3/#lengths
        "cm", "mm", "q", "in", "pt", "pc", "px",
        // font-relative length units: https://drafts.csswg.org/css-values-4/#font-relative-lengths
        "em", "rem", "ex", "rex", "cap", "rcap", "ch", "rch", "ic", "ric", "lh", "rlh",
        // viewport-percentage lengths: https://drafts.csswg.org/css-values-4/#viewport-relative-lengths
        "vw", "svw", "lvw", "dvw", "vh", "svh", "lvh", "dvh", "vi", "svi", "lvi", "dvi", "vb",
        "svb", "lvb", "dvb", "vmin", "svmin", "lvmin", "dvmin", "vmax", "svmax", "lvmax", "dvmax",
        // container-relative lengths: https://drafts.csswg.org/css-contain-3/#container-lengths
        "cqw", "cqh", "cqi", "cqb", "cqmin", "cqmax",
    ];

    pub const ANGLE: &[&str] = &["deg", "grad", "rad", "turn"]; //  https://www.w3.org/TR/css-values-3/#angles
    pub const TIME: &[&str] = &["s", "ms"]; // https://www.w3.org/TR/css-values-3/#time
    pub const FREQUENCY: &[&str] = &["hz", "khz"]; // https://www.w3.org/TR/css-values-3/#frequency
    pub const RESOLUTION: &[&str] = &["dpi", "dpcm", "dppx", "x"]; // https://www.w3.org/TR/css-values-3/#resolution
    pub const FLEX: &[&str] = &["fr"]; // https://drafts.csswg.org/css-grid/#fr-unit
    pub const DECIBEL: &[&str] = &["db"]; // https://www.w3.org/TR/css3-speech/#mixing-props-voice-volume
    pub const SEMITONES: &[&str] = &["st"]; // https://www.w3.org/TR/css3-speech/#voice-props-voice-pitch
}
