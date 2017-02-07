#![feature(plugin, custom_derive, static_in_const)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate image;
extern crate rusttype;
#[macro_use]
extern crate slog;
extern crate slog_term;

pub mod forms;
pub mod color;
pub mod response;

use slog::DrainExt;
use rocket::config::{Config, Environment};
use rocket::State;
use std::env;

const FONT_DATA: &[u8] = include_bytes!("../FiraSans-Light.ttf");

#[get("/?<image>")]
fn index(image: forms::ImageForm, log: State<slog::Logger>) -> response::Image {
    let font_collection = rusttype::FontCollection::from_bytes(FONT_DATA);
    let font = font_collection.into_font().unwrap();

    let scale = rusttype::Scale::uniform(32.0);
    let offset = rusttype::point(6.0, font.v_metrics(scale).ascent);

    let color = image.color.unwrap_or(color::RGB::black());
    let contrast = color.contrast();

    info!(log, "generating image";
          "width" => format!("{}", image.width),
          "height" => format!("{}", image.height),
          "color" => format!("{}", color));

    let mut imgbuf: image::RgbImage = image::ImageBuffer::from_pixel(
        image.width.into(),
        image.height.into(),
        image::Rgb([color.red, color.green, color.blue])
    );

    let text = format!("{}×{}", image.width, image.height);

    for glyph in font.layout(&text, scale, offset) {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as u32 + bounding_box.min.x as u32;
                let y = y as u32 + bounding_box.min.y as u32;

                if (x < image.width.into()) && (y < image.height.into()) {
                    let color = match contrast {
                        color::Contrast::Dark => color.darken(v),
                        color::Contrast::Light => color.lighten(v),
                    };

                    imgbuf.put_pixel(x, y, image::Rgb([color.red, color.green, color.blue]));
                }
            })
        }
    }

    let mut response_image = response::Image::new();

    image::png::PNGEncoder::new(&mut response_image)
        .encode(&imgbuf.into_raw(), image.width.into(), image.height.into(), image::RGB(8))
        .unwrap();

    response_image
}

fn port() -> u16 {
    match env::var("PORT") {
        Ok(val) => val.parse().unwrap_or(8000),
        Err(_) => 8000
    }
}

fn main() {
    let environment = Environment::active().expect("Unable to get active environment");
    let port = port();
    let config = Config::build(environment)
        .address("0.0.0.0")
        .port(port)
        .unwrap();

    let log = slog::Logger::root(slog_term::streamer().stdout().build().fuse(), o!("env" => format!("{}", environment)));

    info!(log, "started"; "port" => port);

    rocket::custom(config, false)
        .mount("/", routes![index])
        .manage(log)
        .launch();
}

