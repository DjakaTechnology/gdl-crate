extern crate image;
use image::{Rgba, DynamicImage};
extern crate imageproc;
extern crate rusttype;
use wasm_bindgen::prelude::*;
use imageproc::drawing::draw_text_mut;
use imageproc::morphology::dilate_mut;
use imageproc::distance_transform::Norm;
use rusttype::{FontCollection, Scale};
use crate::{PhotonImage, helpers, Rgb};

pub fn draw_text(img: &mut PhotonImage, text: &str, x: u32, y:u32, font: &str, font_size: f32) {
        
    let mut image = helpers::dyn_image_from_raw(&img).to_rgba();

    let mut image2 : DynamicImage = DynamicImage::new_luma8(
            image.width(), image.height());

    // include_bytes! only takes a string literal
    let font_vec = match font {
        "Roboto-Regular" => Vec::from(include_bytes!("../fonts/Roboto-Regular.ttf") as &[u8]),
        "Lato-Regular" => Vec::from(include_bytes!("../fonts/Lato-Regular.ttf") as &[u8]),
        "Lato-Regular" => Vec::from(include_bytes!("../fonts/Lato-Bold.ttf") as &[u8]),
        "Roboto-Light" => Vec::from(include_bytes!("../fonts/Roboto-Light.ttf") as &[u8]),
        "Roboto-Bold" => Vec::from(include_bytes!("../fonts/Roboto-Bold.ttf") as &[u8]),
        "Roboto-Black" => Vec::from(include_bytes!("../fonts/Roboto-Black.ttf") as &[u8]),
        "Roboto-Thin" => Vec::from(include_bytes!("../fonts/Roboto-Thin.ttf") as &[u8]),
        _ => Vec::from(include_bytes!("../fonts/Roboto-Bold.ttf") as &[u8])
    };

    let font = FontCollection::from_bytes(font_vec).unwrap().into_font().unwrap();
    let height = font_size;
    let scale = Scale { x: height * 1.0, y: height };
    let white = Rgb{r: 255, g: 255, b: 255};
    let black = Rgb{r: 0, g: 0, b:0};
    draw_text_mut(&mut image2, Rgba([255u8, 255u8, 255u8, 255u8]), x, y, scale, &font, text);

    let mut image2 = image2.to_luma();
    dilate_mut(&mut image2, Norm::LInf, 4u8);

    draw_text_mut(&mut image, Rgba([255u8, 255u8, 255u8, 255u8]), x + 10, y - 10, scale, &font, text);
    let dynimage = image::ImageRgba8(image);
    img.raw_pixels = dynimage.raw_pixels();
    }

pub fn draw_text_with_border(img: &mut PhotonImage, text: &str, x: u32, y: u32) {
    let mut image = helpers::dyn_image_from_raw(&img).to_rgba();

    let mut image2 : DynamicImage = DynamicImage::new_luma8(
        image.width(), image.height());

    let font = Vec::from(include_bytes!("../fonts/Roboto-Black.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
    let height = 90f32;
    let scale = Scale { x: height * 1.0, y: height };
    draw_text_mut(&mut image2, Rgba([255u8, 255u8, 255u8, 255u8]), x, y, scale, &font, text);

    let mut image2 = image2.to_luma();
    dilate_mut(&mut image2, Norm::LInf, 14u8);

    // Add a border to the text.
    for x in 0..image2.width() {
        for y in 0..image2.height() {
            let pixval = 255 - image2.get_pixel(x, y).data[0];
            if pixval != 255 {
                let new_pix = Rgba([234, 23, 123, 255]);
                image.put_pixel(x, y, new_pix);
            }
        }
    }
    // pink
    // draw_text_mut(&mut image, Rgba([244u8, 36u8, 154u8, 255u8]), x + 10, y - 10, scale, &font, text);

    draw_text_mut(&mut image, Rgba([193u8, 255u8, 255u8, 255u8]), x + 10, y - 10, scale, &font, text);

    let dynimage = image::ImageRgba8(image);
    img.raw_pixels = dynimage.raw_pixels();
}