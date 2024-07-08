use std::io::Cursor;
use wasm_bindgen::prelude::*;
use image::{DynamicImage, ImageFormat, Rgba};
use rusttype::{Scale, point};
use base64::{engine::general_purpose, Engine as _};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn str_to_png(str:&str,win_h: u32,font_data:&[u8])  -> String  {
    let text = str;
    let font = rusttype::Font::try_from_vec(Vec::from(font_data)).unwrap();
    let scale = Scale::uniform((win_h as f64 * 0.8) as f32);

    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font
        .layout(text, scale, point(10.0, 10.0 + v_metrics.ascent))
        .collect();

    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    let mut image = DynamicImage::new_rgba8(glyphs_width + (win_h as f64 * 0.1) as u32, glyphs_height + (win_h as f64 * 0.1) as u32).to_rgba8();

    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                image.put_pixel(
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    Rgba([255, 255, 0, (v * 255.0) as u8]),
                )
            });
        }
    }

    let mut buffer = Cursor::new(Vec::new());
    image.write_to(&mut buffer, ImageFormat::Png).unwrap();
    let base64_string = general_purpose::STANDARD.encode(buffer.get_ref());
    // データURL形式で返す
    format!("data:image/png;base64,{}", base64_string)

}