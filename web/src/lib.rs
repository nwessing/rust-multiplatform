extern crate wasm_bindgen;
extern crate core;
extern crate web_sys;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;

use core::pong::{
    Pong,
    ImageBuffer,
    PongInput,
    VerticalOrigin
};

static mut PONG: Option<Pong> = None;
static mut IMAGE_BUFFER: Option<ImageBuffer> = None;

#[wasm_bindgen]
pub fn run(delta_time: f32, move_up: bool, move_down: bool) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let player_input = if move_up {
        Some(PongInput::Up)
    } else if move_down {
        Some(PongInput::Down)
    } else {
        None
    };

    unsafe {
        if PONG.is_none() {
            PONG = Some(Pong::new());
        }

        if IMAGE_BUFFER.is_none() {
            let mut image_buffer = ImageBuffer::new(600, 600, VerticalOrigin::Top, 0, 1, 2);
            for i in 0..image_buffer.data.len() / 4 {
                image_buffer.data[(i * 4) + 3] = 255;
            }

            IMAGE_BUFFER = Some(image_buffer);

        }

        let pong = PONG.as_mut().unwrap();
        let image_buffer = IMAGE_BUFFER.as_mut().unwrap();
        pong.update(player_input, delta_time, image_buffer);

        let image_array = Clamped(&mut image_buffer.data as &mut [u8]);
        let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(image_array, image_buffer.width as u32, image_buffer.height as u32).unwrap();
        context.put_image_data(&image_data, 0.0, 0.0).unwrap();
    }
}
