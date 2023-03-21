
use hex::FromHex;
use palette::{FromColor, Hsl, Srgb};

pub fn hex_to_hsl(hex_input: &String) -> Hsl {
    match <[u8; 3]>::from_hex(hex_input) {
        Ok([r, g, b]) => Hsl::from_color(Srgb::from_components((
            (r as f32) / 255.0,
            (g as f32) / 255.0,
            (b as f32) / 255.0,
        ))),
        Err(_) => panic!("failed to decode the color {}", hex_input),
    }
}

pub fn hsl_to_hex(color: &Hsl) -> String {
    let srgb = Srgb::from_color(*color);
    format!(
        "{:02X}{:02X}{:02X}",
        (srgb.red * 255.0).round() as u8,
        (srgb.green * 255.0).round() as u8,
        (srgb.blue * 255.0).round() as u8
    )
}

#[cfg(test)]
mod tests {
    use crate::helpers::{hex_to_hsl, hsl_to_hex};

    #[test]
    fn color_roundtrip() {
        for color in vec!["123456", "ABCDEF", "231171", "A2B4C6"] {
            let hex_input = String::from(color);
            let hsl_color = hex_to_hsl(&hex_input);
            let hex_output = hsl_to_hex(&hsl_color);
            assert_eq!(hex_input, hex_output);
        }
    }
}
