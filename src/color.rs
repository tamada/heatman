use std::fmt::Display;
use std::str::FromStr;
use image::Rgba;

use crate::{Error, Result};

/// The names of the available colors for [`ColorMap`].
pub const COLOR_NAMES: [&str; 14] = [
    "full-color",
    "red", "green", "blue", "yellow", "cyan", "magenta",
    "redblack", "greenblack", "blueblack", "yellowblack", "cyanblack", "magentablack",
    "gray",
];

/// Maps a value between 0 and 1 to a color.
///
/// The string representation is one of the followings.
///
/// * `full-color`: the hue from blue (0) to red (1).
/// * `<name>[:<from>-<to>]`: the gradient of the named color.
///     * `red`, `green`, `blue`, `yellow`, `cyan`, and `magenta` are from white (0) to the color (1).
///     * `redblack`, `greenblack`, ..., and `magentablack` are from black (0) to the color (1).
///     * `gray` is from white (0) to black (1).
///     * `from` and `to` are the strength (0-255) of the color for the values 0 and 1 (default: `0-255`).
///       If `from` is greater than `to`, the direction of the gradient is reversed.
/// * `<RRGGBB>-<RRGGBB>`: the gradient between the two colors in hex, e.g., `ff0000-ffffff`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorMap {
    /// Maps the value to the hue from blue (0) to red (1).
    #[default]
    FullColor,
    /// Maps the value to the linear gradient between two colors.
    Gradient {
        /// The color for the value 0.
        from: [u8; 3],
        /// The color for the value 1.
        to: [u8; 3],
    },
}

impl ColorMap {
    /// Converts a value between 0 and 1 to an RGBA color.
    /// The value out of the range is clamped.
    pub fn map(&self, value: f64) -> Rgba<u8> {
        match self {
            ColorMap::FullColor => crate::convert(value),
            ColorMap::Gradient { from, to } => {
                let v = value.clamp(0.0, 1.0);
                let c = |i: usize| {
                    let (from, to) = (from[i] as f64, to[i] as f64);
                    (from + v * (to - from)).round() as u8
                };
                Rgba([c(0), c(1), c(2), 255])
            }
        }
    }
}

/// The base of the named color for the strength 0.
#[derive(Clone, Copy)]
enum Base {
    White,
    Black,
}

/// Returns the channels (red, green, blue) of the named color and its base.
/// `gray` has no channels, so that it becomes white to black.
fn color_of(name: &str) -> Option<([bool; 3], Base)> {
    let (name, base) = match name.strip_suffix("black") {
        Some(name) if !name.is_empty() => (name, Base::Black),
        _ => (name, Base::White),
    };
    let channels = match name {
        "red" => [true, false, false],
        "green" => [false, true, false],
        "blue" => [false, false, true],
        "yellow" => [true, true, false],
        "cyan" => [false, true, true],
        "magenta" => [true, false, true],
        "gray" | "grey" if matches!(base, Base::White) => [false, false, false],
        _ => return None,
    };
    Some((channels, base))
}

/// Returns the RGB of the named color with the given strength.
fn rgb_of(channels: &[bool; 3], base: Base, strength: u8) -> [u8; 3] {
    channels.map(|enabled| match (base, enabled) {
        (Base::White, true) => 255,
        (Base::White, false) => 255 - strength,
        (Base::Black, true) => strength,
        (Base::Black, false) => 0,
    })
}

fn parse_strength(s: &str, item: &str) -> Result<u8> {
    item.trim().parse::<u8>()
        .map_err(|e| Error::ParseColor(s.to_string(), format!("{item}: strength should be an integer between 0 and 255 ({e})")))
}

fn parse_hex(s: &str) -> Option<[u8; 3]> {
    if s.len() != 6 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let c = |i: usize| u8::from_str_radix(&s[i..i + 2], 16).ok();
    Some([c(0)?, c(2)?, c(4)?])
}

fn parse_named(s: &str, name: &str, range: Option<&str>) -> Result<ColorMap> {
    let (channels, base) = color_of(name)
        .ok_or_else(|| Error::ParseColor(s.to_string(), format!("unknown color name (available: {}, or RRGGBB-RRGGBB)", COLOR_NAMES.join(", "))))?;
    let (from, to) = match range {
        None => (0, 255),
        Some(range) => {
            let (from, to) = range.split_once('-')
                .ok_or_else(|| Error::ParseColor(s.to_string(), "the range should be specified as 'from-to' with a hyphen, e.g., red:0-200".to_string()))?;
            (parse_strength(s, from)?, parse_strength(s, to)?)
        }
    };
    Ok(ColorMap::Gradient {
        from: rgb_of(&channels, base, from),
        to: rgb_of(&channels, base, to),
    })
}

impl FromStr for ColorMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let lower = s.trim().to_lowercase();
        let (name, range) = match lower.split_once(':') {
            Some((name, range)) => (name, Some(range)),
            None => (lower.as_str(), None),
        };
        if name == "full-color" {
            return match range {
                None => Ok(ColorMap::FullColor),
                Some(_) => Err(Error::ParseColor(s.to_string(), "full-color does not accept the range".to_string())),
            };
        }
        if range.is_none() && let Some((from, to)) = name.split_once('-')
            && let (Some(from), Some(to)) = (parse_hex(from), parse_hex(to)) {
            return Ok(ColorMap::Gradient { from, to });
        }
        parse_named(s, name, range)
    }
}

impl Display for ColorMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorMap::FullColor => write!(f, "full-color"),
            ColorMap::Gradient { from, to } => write!(f, "{:02x}{:02x}{:02x}-{:02x}{:02x}{:02x}",
                from[0], from[1], from[2], to[0], to[1], to[2]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gradient(from: [u8; 3], to: [u8; 3]) -> ColorMap {
        ColorMap::Gradient { from, to }
    }

    #[test]
    fn test_parse() {
        assert_eq!("full-color".parse::<ColorMap>().unwrap(), ColorMap::FullColor);
        assert_eq!("red".parse::<ColorMap>().unwrap(), gradient([255, 255, 255], [255, 0, 0]));
        assert_eq!("redblack".parse::<ColorMap>().unwrap(), gradient([0, 0, 0], [255, 0, 0]));
        assert_eq!("Yellow".parse::<ColorMap>().unwrap(), gradient([255, 255, 255], [255, 255, 0]));
        assert_eq!("gray".parse::<ColorMap>().unwrap(), gradient([255, 255, 255], [0, 0, 0]));
        assert_eq!("gray:255-0".parse::<ColorMap>().unwrap(), gradient([0, 0, 0], [255, 255, 255]));
        assert_eq!("cyanblack:10-200".parse::<ColorMap>().unwrap(), gradient([0, 10, 10], [0, 200, 200]));
        assert_eq!("blue:55-255".parse::<ColorMap>().unwrap(), gradient([200, 200, 255], [0, 0, 255]));
        assert_eq!("FF0000-ffffff".parse::<ColorMap>().unwrap(), gradient([255, 0, 0], [255, 255, 255]));
    }

    #[test]
    fn test_parse_errors() {
        assert!("full-color:0-255".parse::<ColorMap>().is_err());
        assert!("purple".parse::<ColorMap>().is_err());
        assert!("black".parse::<ColorMap>().is_err());
        assert!("grayblack".parse::<ColorMap>().is_err());
        assert!("red:0-256".parse::<ColorMap>().is_err());
        assert!("red:-1-255".parse::<ColorMap>().is_err());
        assert!("red:100".parse::<ColorMap>().is_err());
        assert!("ff0000".parse::<ColorMap>().is_err());
        assert!("ff000-ffffff".parse::<ColorMap>().is_err());
        assert!("ff0000-ffffff:0-255".parse::<ColorMap>().is_err());
    }

    #[test]
    fn test_map() {
        let red: ColorMap = "red".parse().unwrap();
        assert_eq!(red.map(0.0), Rgba([255, 255, 255, 255]));
        assert_eq!(red.map(1.0), Rgba([255, 0, 0, 255]));
        assert_eq!(red.map(2.0), Rgba([255, 0, 0, 255]));
        let redblack: ColorMap = "redblack".parse().unwrap();
        assert_eq!(redblack.map(0.0), Rgba([0, 0, 0, 255]));
        assert_eq!(redblack.map(1.0), Rgba([255, 0, 0, 255]));
        let gray: ColorMap = "gray".parse().unwrap();
        assert_eq!(gray.map(0.5), Rgba([128, 128, 128, 255]));
        assert_eq!(gray.map(1.0), Rgba([0, 0, 0, 255]));
        assert_eq!(ColorMap::FullColor.map(0.3), crate::convert(0.3));
    }

    #[test]
    fn test_display_round_trip() {
        for s in ["full-color", "ff0000-ffffff", "000000-00ffff"] {
            assert_eq!(s.parse::<ColorMap>().unwrap().to_string(), s);
        }
    }
}
