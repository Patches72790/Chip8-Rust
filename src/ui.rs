use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::cpu::Cpu;

static PIXEL_OFF_COLOR: &str = "#000000";
static PIXEL_ON_COLOR: &str = "#FFFFFF";
static PIXEL_SIZE: usize = 10;

//#[wasm_bindgen]
pub fn run_chip8() -> Result<(), JsValue> {
    let mut cpu = Cpu::new(false);
    let width = cpu.width();
    let height = cpu.height();

    let context = get_context(&width, &height);
    //cpu.load_instructions();
    //let instructions = cpu.disassemble();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        cpu.tick();
        draw_pixels(&cpu, &context);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

fn draw_pixels(cpu: &Cpu, context: &CanvasRenderingContext2d) {
    context.begin_path();

    context.set_stroke_style(&JsValue::from_str(PIXEL_OFF_COLOR));

    // vertical lines
    for i in 0..cpu.width() {
        context.move_to((i * (PIXEL_SIZE + 1) + 1) as f64, 0.0);
        context.line_to(
            (i * (PIXEL_SIZE + 1) + 1) as f64,
            ((PIXEL_SIZE + 1) * cpu.height() + 1) as f64,
        );
    }
    // horizontal lines
    for i in 0..cpu.height() {
        context.move_to(0.0, (i * (PIXEL_SIZE + 1) + 1) as f64);
        context.line_to(
            ((PIXEL_SIZE + 1) * cpu.width() + 1) as f64,
            (i * (PIXEL_SIZE + 1) + 1) as f64,
        );
    }
    context.stroke();
}

fn get_context(width: &usize, height: &usize) -> web_sys::CanvasRenderingContext2d {
    let canvas = document().get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(((PIXEL_SIZE + 1) * *width + 1) as u32);
    canvas.set_height(((PIXEL_SIZE + 1) * *height + 1) as u32);

    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}

fn window() -> web_sys::Window {
    web_sys::window().expect("Error finding window!")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Request animation frame error");
}

fn document() -> web_sys::Document {
    window().document().expect("Error getting window document")
}
