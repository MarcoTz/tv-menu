use crate::Error;
use eframe::egui::Color32;

pub fn parse_color(input: &str) -> Result<Color32, Error> {
    if input.starts_with("#") {
        parse_hex(input)
    } else if input.starts_with("rgba") {
        parse_rgba(input)
    } else if input.starts_with("rgb") {
        parse_rgb(input)
    } else {
        Err(Error::InvalidColor(input.to_owned()))
    }
}

fn parse_hex(input: &str) -> Result<Color32, Error> {
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
    Ok(Color32::from_rgba_unmultiplied(
        hex_red, hex_blue, hex_green, hex_alpha,
    ))
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

fn parse_rgba(input: &str) -> Result<Color32, Error> {
    let mut color = input.replace("rgba(", "");
    color.remove(color.len() - 1);
    let mut parts = color.split(",");
    let red = parts
        .next()
        .ok_or(Error::InvalidColor(input.to_owned()))?
        .parse::<u8>()
        .map_err(|_| Error::InvalidColor(input.to_owned()))?;
    let green = parts
        .next()
        .ok_or(Error::InvalidColor(input.to_owned()))?
        .parse::<u8>()
        .map_err(|_| Error::InvalidColor(input.to_owned()))?;
    let blue = parts
        .next()
        .ok_or(Error::InvalidColor(input.to_owned()))?
        .parse::<u8>()
        .map_err(|_| Error::InvalidColor(input.to_owned()))?;
    let alpha = parts
        .next()
        .ok_or(Error::InvalidColor(input.to_owned()))?
        .parse::<u8>()
        .map_err(|_| Error::InvalidColor(input.to_owned()))?;
    Ok(Color32::from_rgba_unmultiplied(red, green, blue, alpha))
}

fn parse_rgb(input: &str) -> Result<Color32, Error> {
    let mut color = input.replace("rgb(", "");
    color.remove(color.len() - 1);
    let mut parts = color.split(",");
    let red = parts
        .next()
        .ok_or(Error::InvalidColor(input.to_owned()))?
        .parse::<u8>()
        .map_err(|_| Error::InvalidColor(input.to_owned()))?;
    let green = parts
        .next()
        .ok_or(Error::InvalidColor(input.to_owned()))?
        .parse::<u8>()
        .map_err(|_| Error::InvalidColor(input.to_owned()))?;
    let blue = parts
        .next()
        .ok_or(Error::InvalidColor(input.to_owned()))?
        .parse::<u8>()
        .map_err(|_| Error::InvalidColor(input.to_owned()))?;
    Ok(Color32::from_rgb(red, green, blue))
}

#[cfg(test)]
mod color_tests {
    use super::parse_color;
    use eframe::egui::Color32;

    #[test]
    fn parse_hex() {
        let result = parse_color("#ffffff").unwrap();
        let expected = Color32::from_rgb(255, 255, 255);
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_hex_upper() {
        let result = parse_color("#FFFFFF").unwrap();
        let expected = Color32::from_rgb(255, 255, 255);
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_hex_alpha() {
        let result = parse_color("#aaaaaaaa").unwrap();
        let expected = Color32::from_rgba_unmultiplied(170, 170, 170, 170);
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_rgba() {
        let result = parse_color("rgba(255,255,255,255)").unwrap();
        let expected = Color32::from_rgba_unmultiplied(255, 255, 255, 255);
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_rbg() {
        let result = parse_color("rgb(255,255,255)").unwrap();
        let expected = Color32::from_rgb(255, 255, 255);
        assert_eq!(result, expected)
    }
}
