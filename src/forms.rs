use rocket::request::FromFormValue;
use color::RGB;
use std::fmt;

#[derive(FromForm, Debug)]
pub struct ImageForm {
    pub width: Dimension,
    pub height: Dimension,
    pub color: Option<RGB>,
}

const MAX_DIMENSION: usize = 10_000;

#[derive(Debug, Copy, Clone)]
pub struct Dimension(usize);

impl<'v> FromFormValue<'v> for Dimension {
    type Error = &'v str;

    fn from_form_value(form_value: &'v str) -> Result<Dimension, &'v str> {
        match usize::from_form_value(form_value) {
            Ok(num) if num < MAX_DIMENSION => Ok(Dimension(num)),
            _ => Err(form_value)
        }
    }
}

impl From<Dimension> for u32 {
    fn from(dimension: Dimension) -> u32 {
        match dimension {
            Dimension(n) => n as u32
        }
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &Dimension(n) = self;
        write!(f, "{}", n)
    }
}
