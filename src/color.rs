use rocket::request::FromFormValue;
use std::str;
use std::fmt;

pub enum Contrast {
    Dark,
    Light
}

#[derive(Debug, Copy, Clone)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RGB {
    pub fn black() -> RGB {
        RGB { red: 0, green: 0, blue: 0 }
    }

    pub fn white() -> RGB {
        RGB { red: 255, green: 255, blue: 255 }
    }

    pub fn contrast(self) -> Contrast {
        let yiq: usize = ((self.red as usize * 299) + (self.green as usize * 587) + (self.blue as usize * 114)) / 1000;
        if yiq >= 128 {
            Contrast::Dark
        } else {
            Contrast::Light
        }
    }

    pub fn lighten(self, n: f32) -> RGB {
        RGB {
            red: self.red.checked_add((255.0 * n) as u8).unwrap_or(255),
            green: self.green.checked_add((255.0 * n) as u8).unwrap_or(255),
            blue: self.blue.checked_add((255.0 * n) as u8).unwrap_or(255),
        }
    }

    pub fn darken(self, n: f32) -> RGB {
        RGB {
            red: (self.red as f32 * (1.0 - n)) as u8,
            green: (self.green as f32 * (1.0 - n)) as u8,
            blue: (self.blue as f32 * (1.0 - n)) as u8,
        }
    }
}

impl<'v> FromFormValue<'v> for RGB {
    type Error = &'v str;

    fn from_form_value(form_value: &'v str) -> Result<RGB, &'v str> {
        if form_value.len() != 6 {
            return Err(form_value);
        }

        match u32::from_str_radix(form_value, 16) {
            Ok(n) if n <= 16777216 => {
                let red = (n >> 16 & 255) as u8;
                let green = (n >> 8 & 255) as u8;
                let blue = (n & 255) as u8;

                Ok(RGB { red: red, green: green, blue: blue})
            },
            _ => Err(form_value)
        }
    }
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rgb({red} {green} {blue})", red=self.red, blue=self.blue, green=self.green)
    }
}

#[cfg(test)]
mod tests {
    use rocket::request::FromFormValue;
    use super::RGB;

    #[test]
    fn six_digit_valid() {
        let rgb = RGB::from_form_value("ffffff");
        assert!(rgb.is_ok());
    }

    #[test]
    fn number_too_big() {
        let rgb = RGB::from_form_value("ffffff0");
        assert!(rgb.is_err());
    }

    #[test]
    fn red_channel() {
        let rgb = RGB::from_form_value("aabbcc").unwrap();
        assert_eq!(rgb.red, 170);
    }

    #[test]
    fn green_channel() {
        let rgb = RGB::from_form_value("aabbcc").unwrap();
        assert_eq!(rgb.green, 187);
    }

    #[test]
    fn blue_channel() {
        let rgb = RGB::from_form_value("aabbcc").unwrap();
        assert_eq!(rgb.blue, 204);
    }
}
