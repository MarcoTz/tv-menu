use crate::Error;

#[derive(PartialEq, Eq, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    /// Black, #000000ff;
    pub const BLACK: Self = Self {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 255,
    };

    /// White, #ffffffff;
    pub const WHITE: Self = Self {
        red: 255,
        green: 255,
        blue: 255,
        alpha: 255,
    };

    /// Transparent, #00000000;
    pub const TRANSPARENT: Self = Self {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 0,
    };

    /// create Rgb color #rgbff
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
            alpha: 255,
        }
    }

    /// create rgba color #rgba
    #[must_use]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
            alpha: a,
        }
    }
}

pub fn parse_color(input: &str) -> Result<Color, Error> {
    if input.starts_with('#') {
        parse_hex(input)
    } else if input.starts_with("rgba") {
        parse_rgba(input)
    } else if input.starts_with("rgb") {
        parse_rgb(input)
    } else {
        Err(Error::InvalidColor(input.to_owned()))
    }
}

fn parse_hex(input: &str) -> Result<Color, Error> {
    if input.len() != 7 && input.len() != 9 {
        return Err(Error::InvalidColor(input.to_owned()));
    }
    let input = input.replace('#', "");
    let mut hex_vals = vec![];
    for ch in input.chars() {
        hex_vals.push(hex_to_digit(ch)?);
    }
    let mut hex_red = hex_vals.remove(0) * 16;
    hex_red += hex_vals.remove(0);
    let mut hex_blue = hex_vals.remove(0) * 16;
    hex_blue += hex_vals.remove(0);
    let mut hex_green = hex_vals.remove(0) * 16;
    hex_green += hex_vals.remove(0);
    let mut hex_alpha = 255;
    if !hex_vals.is_empty() {
        hex_alpha = hex_vals.remove(0) * 16;
        hex_alpha += hex_vals.remove(0);
    }
    Ok(Color::rgba(hex_red, hex_blue, hex_green, hex_alpha))
}

fn hex_to_digit(ch: char) -> Result<u8, Error> {
    match ch.to_ascii_lowercase() {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        'a' => Ok(10),
        'b' => Ok(11),
        'c' => Ok(12),
        'd' => Ok(13),
        'e' => Ok(14),
        'f' => Ok(15),
        _ => Err(Error::InvalidColor(format!("Hex {ch}"))),
    }
}

fn parse_rgba(input: &str) -> Result<Color, Error> {
    let mut color = input.replace("rgba(", "");
    color.remove(color.len() - 1);
    let mut parts = color.split(',');
    let red = parts
        .next()
        .ok_or_else(|| Error::InvalidColor(input.to_owned()))?
        .parse::<u8>()
        .map_err(|_| Error::InvalidColor(input.to_owned()))?;
    let green = parts
        .next()
        .ok_or_else(|| Error::InvalidColor(input.to_owned()))?
        .parse::<u8>()
        .map_err(|_| Error::InvalidColor(input.to_owned()))?;
    let blue = parts
        .next()
        .ok_or_else(|| Error::InvalidColor(input.to_owned()))?
        .parse::<u8>()
        .map_err(|_| Error::InvalidColor(input.to_owned()))?;
    let alpha = parts
        .next()
        .ok_or_else(|| Error::InvalidColor(input.to_owned()))?
        .parse::<u8>()
        .map_err(|_| Error::InvalidColor(input.to_owned()))?;
    Ok(Color::rgba(red, green, blue, alpha))
}

fn parse_rgb(input: &str) -> Result<Color, Error> {
    let mut color = input.replace("rgb(", "");
    color.remove(color.len() - 1);
    let mut parts = color.split(',');
    let red = parts
        .next()
        .ok_or_else(|| Error::InvalidColor(input.to_owned()))?
        .parse::<u8>()
        .map_err(|_| Error::InvalidColor(input.to_owned()))?;
    let green = parts
        .next()
        .ok_or_else(|| Error::InvalidColor(input.to_owned()))?
        .parse::<u8>()
        .map_err(|_| Error::InvalidColor(input.to_owned()))?;
    let blue = parts
        .next()
        .ok_or_else(|| Error::InvalidColor(input.to_owned()))?
        .parse::<u8>()
        .map_err(|_| Error::InvalidColor(input.to_owned()))?;
    Ok(Color::rgb(red, green, blue))
}

#[cfg(test)]
mod color_tests {
    use super::{Color, parse_color};

    #[test]
    fn parse_hex() {
        let result = parse_color("#ffffff").unwrap();
        let expected = Color::rgb(255, 255, 255);
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_hex_upper() {
        let result = parse_color("#FFFFFF").unwrap();
        let expected = Color::rgb(255, 255, 255);
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_hex_alpha() {
        let result = parse_color("#aaaaaaaa").unwrap();
        let expected = Color::rgba(170, 170, 170, 170);
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_rgba() {
        let result = parse_color("rgba(255,255,255,255)").unwrap();
        let expected = Color::rgba(255, 255, 255, 255);
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_rbg() {
        let result = parse_color("rgb(255,255,255)").unwrap();
        let expected = Color::rgb(255, 255, 255);
        assert_eq!(result, expected)
    }
}
