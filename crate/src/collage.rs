extern crate image;
use image::{GenericImage, GenericImageView, Rgba, DynamicImage};
extern crate imageproc;
extern crate rusttype;
use wasm_bindgen::prelude::*;
use imageproc::drawing::draw_text_mut;
use imageproc::morphology::dilate_mut;
use imageproc::distance_transform::Norm;
use rusttype::{FontCollection, Scale};
use crate::{PhotonImage, helpers, Rgb};
use image::FilterType::Nearest;
use crate::text::*;
use crate::elements::*;

/// Add text to an image.
#[wasm_bindgen]
pub fn two_grid(mut photon_img: PhotonImage, mut photon_img2: PhotonImage, width: u32, height: u32) -> PhotonImage {
    let image = helpers::dyn_image_from_raw(&photon_img);
    let image2 = helpers::dyn_image_from_raw(&photon_img2);

    // distribute the width evenly by allocating the same space to both images
    let img_width = width / 2;
    let mut imgs = vec![image, image2];
    resize_imgs(&mut imgs, img_width, height);

    let mut container_img : DynamicImage = DynamicImage::new_rgba8(width, height);

    image::imageops::overlay(&mut container_img, &imgs[0], 0, 0);
    image::imageops::overlay(&mut container_img, &imgs[1], img_width, 0);

    // return the collage
    return PhotonImage {raw_pixels: container_img.raw_pixels(), width: container_img.width(), height: container_img.height()};
}

/// Resize images in a vec, returns a new vec with resized images.
fn resize_imgs(mut imgs: &mut Vec<DynamicImage>, img_width: u32, img_height: u32) {
    let sampling_filter = image::FilterType::Nearest;

    for i in 0..imgs.len() {
        let item = &imgs[i];
        let image = image::ImageRgba8(image::imageops::resize(item, img_width, img_height, sampling_filter));

        imgs[i] = image;
    }
}

/// Add text to an image.
#[wasm_bindgen]
pub fn four_grid_text(mut photon_img: PhotonImage, photon_img2: PhotonImage, width: u32, height: u32) -> PhotonImage {
    let image = helpers::dyn_image_from_raw(&photon_img);
    let image2 = helpers::dyn_image_from_raw(&photon_img2);

    // distribute the width evenly by allocating the same space to both images
    let img_width = width / 2;
    let img_height = height / 2;

    let sampling_filter = image::FilterType::Nearest;
    let image = image::ImageRgba8(image::imageops::resize(&image, img_width, img_height, sampling_filter));
    let image2 = image::ImageRgba8(image::imageops::resize(&image2, img_width, img_height, sampling_filter));

    let mut container_img : DynamicImage = DynamicImage::new_rgba8(width, height);

    image::imageops::overlay(&mut container_img, &image, 0, 0);
    image::imageops::overlay(&mut container_img, &image2, image.width(), image.height());

    // return the collage
    let mut photon_img = PhotonImage::new_from_rawpixels(container_img.raw_pixels(), container_img.width(), container_img.height());
    let white = Rgb{r: 255, g: 255, b: 255};
    let black = Rgb{r: 0, g: 0, b:0};
    let lilac = Rgb{r: 204, g: 195, b: 240};
    let yellow = Rgb{ r: 255, g: 226, b: 98};
    draw_solid_rect(&mut photon_img, &yellow, img_height, img_width, image.width() as i32, 0);
    draw_solid_rect(&mut photon_img, &lilac, img_height, img_width, 0, image.height() as i32);
    draw_text(&mut photon_img, "Daisies In the Underground", image.width() + 30, img_height / 2, "Roboto-Bold", 30.0);
    
    return photon_img;

}

/// Add text to an image.
#[wasm_bindgen]
pub fn collage_test(photon_img: PhotonImage, photon_img2: PhotonImage, width: u32, height: u32) -> PhotonImage {
    let image = helpers::dyn_image_from_raw(&photon_img);
    let image2 = helpers::dyn_image_from_raw(&photon_img2);

    // distribute the width evenly by allocating the same space to both images
    let img_width = width / 2;
    let img_height = height / 2;

    let sampling_filter = image::FilterType::Nearest;
    let image = image::ImageRgba8(image::imageops::resize(&image, img_width, img_height, sampling_filter));
    let image2 = image::ImageRgba8(image::imageops::resize(&image2, img_width, img_height, sampling_filter));

    let mut container_img : DynamicImage = DynamicImage::new_rgba8(width, height);

    image::imageops::overlay(&mut container_img, &image, 0, 0);
    image::imageops::overlay(&mut container_img, &image2, img_width, img_height);

    // return the collage
    let mut photon_img = PhotonImage::new_from_rawpixels(container_img.raw_pixels(), container_img.width(), container_img.height());
    let white = Rgb{r: 255, g: 255, b: 255};
    let black = Rgb{r: 0, g: 0, b:0};
    let lilac = Rgb{r: 204, g: 195, b: 240};
    let yellow = Rgb{ r: 255, g: 226, b: 98};
    draw_solid_rect(&mut photon_img, &yellow, img_height, img_width, image.width() as i32, 0);
    draw_solid_rect(&mut photon_img, &lilac, img_height, img_width, 0, image.height() as i32);

    draw_text(&mut photon_img, "Daisies In the Underground", image.width() + 30, img_height / 2, "Roboto-Bold", 30.0);
    
    return photon_img;

}

/// Add text to an image.
#[wasm_bindgen]
pub fn split_imgs_text(photon_img: PhotonImage, photon_img2: PhotonImage, width: u32, height: u32) -> PhotonImage {
    let image = helpers::dyn_image_from_raw(&photon_img);
    let image2 = helpers::dyn_image_from_raw(&photon_img2);

    // distribute the width evenly by allocating the same space to both images
    let img_width = width / 2;
    let img_height = height / 2;

    let sampling_filter = image::FilterType::Nearest;
    let image = image::ImageRgba8(image::imageops::resize(&image, img_width, img_height, sampling_filter));
    let image2 = image::ImageRgba8(image::imageops::resize(&image2, img_width, img_height, sampling_filter));

    let mut container_img : DynamicImage = DynamicImage::new_rgba8(width, height);

    image::imageops::overlay(&mut container_img, &image, img_width, 0);
    image::imageops::overlay(&mut container_img, &image2, image.width(), image.height());

    // return the collage
    let mut photon_img = PhotonImage::new_from_rawpixels(container_img.raw_pixels(), container_img.width(), container_img.height());
    let white = Rgb{r: 255, g: 255, b: 255};
    let black = Rgb{r: 0, g: 0, b:0};
    let lilac = Rgb{r: 204, g: 195, b: 240};
    let yellow = Rgb{ r: 255, g: 226, b: 98};
    draw_solid_rect(&mut photon_img, &white, img_height * 2, img_width, 0, 0);
    draw_text(&mut photon_img, "Life Is An Adventure", 45, img_height / 2, "Roboto-Bold", 30.0);
    
    return photon_img;

}

/// Add text to an image.
#[wasm_bindgen]
pub fn four_grid(photon_img: PhotonImage, photon_img2: PhotonImage, photon_img3: PhotonImage, photon_img4: PhotonImage, width: u32, height: u32) -> PhotonImage {
    let image = helpers::dyn_image_from_raw(&photon_img);
    let image2 = helpers::dyn_image_from_raw(&photon_img2);
    let image3 = helpers::dyn_image_from_raw(&photon_img3);
    let image4 = helpers::dyn_image_from_raw(&photon_img4);

    // distribute the width evenly by allocating the same space to both images
    let img_width = width / 2;
    let img_height = height / 2;

    let mut imgs = vec![image, image2, image3, image4];
    resize_imgs(&mut imgs, img_width, img_height);

    let mut container_img : DynamicImage = DynamicImage::new_rgba8(width, height);

    image::imageops::overlay(&mut container_img, &imgs[0], 0, 0);
    image::imageops::overlay(&mut container_img, &imgs[1], img_width, 0);
    image::imageops::overlay(&mut container_img, &imgs[2], 0, img_height);
    image::imageops::overlay(&mut container_img, &imgs[3], img_width, img_height);

    // return the collage
    let photon_img = PhotonImage {raw_pixels: container_img.raw_pixels(), width: container_img.width(), height: container_img.height()};
    return photon_img;
}

/// Create a triple grid collage graphic.
#[wasm_bindgen]
pub fn triple_grid(photon_img: PhotonImage, photon_img2: PhotonImage, photon_img3: PhotonImage, width: u32, height: u32) -> PhotonImage {
    let image = helpers::dyn_image_from_raw(&photon_img);
    let image2 = helpers::dyn_image_from_raw(&photon_img2);
    let image3 = helpers::dyn_image_from_raw(&photon_img3);

    // distribute the width evenly by allocating the same space to both images
    let img_width = width / 3;
    let mut imgs = vec![image, image2, image3];
    resize_imgs(&mut imgs, img_width, height);

    let mut container_img : DynamicImage = DynamicImage::new_rgba8(width, height);

    image::imageops::overlay(&mut container_img, &imgs[0], 0, 0);
    image::imageops::overlay(&mut container_img, &imgs[1], img_width, 0);
    image::imageops::overlay(&mut container_img, &imgs[2], img_width * 2, 0);

    // return the collage
    return PhotonImage {raw_pixels: container_img.raw_pixels(), width: container_img.width(), height: container_img.height()};
}