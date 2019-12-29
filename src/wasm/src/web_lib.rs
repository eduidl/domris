use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

pub fn ctx() -> web_sys::CanvasRenderingContext2d {
    let canvas = document()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

pub fn game_level_btns() -> web_sys::NodeList {
    document()
        .query_selector_all("#levelBtnGroup>button")
        .unwrap()
}

pub fn game_level_btn(game_level_btns: &web_sys::NodeList, index: u32) -> web_sys::Element {
    game_level_btns
        .get(index)
        .unwrap()
        .dyn_into::<web_sys::Element>()
        .unwrap()
}

pub fn game_toggle_btn() -> web_sys::HtmlElement {
    document()
        .get_element_by_id("gameToggleBtn")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap()
}

pub fn point() -> web_sys::HtmlElement {
    document()
        .get_element_by_id("point")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap()
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}

pub fn cancel_animation_frame(handle: i32) {
    window().cancel_animation_frame(handle).unwrap();
}
