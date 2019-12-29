use std::cell::RefCell;
use std::rc::Rc;

use js_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

mod domris;
mod drawer;
mod tetromino;
mod web_lib;
use self::domris::{Control, Domris};

struct Timer {
    last: f64,
}

impl Timer {
    fn new() -> Self {
        Self { last: -1. }
    }

    fn tick(&mut self) -> f64 {
        let now = js_sys::Date::now();
        let elapsed = now - self.last;
        self.last = now;
        elapsed
    }
}

fn key2control(e: &web_sys::KeyboardEvent) -> Option<Control> {
    match e.key().as_ref() {
        "ArrowRight" | "l" => Some(Control::MoveRight),
        "ArrowLeft" | "h" => Some(Control::MoveLeft),
        "ArrowDown" | "j" => Some(Control::MoveDown),
        "ArrowUp" | "k" => Some(Control::MoveBottom),
        " " => {
            if e.shift_key() {
                Some(Control::RotateLeft)
            } else {
                Some(Control::RotateRight)
            }
        }
        _ => None,
    }
}

#[wasm_bindgen(start)]
pub fn start_app() {
    let domris = Rc::new(RefCell::new(Domris::new()));
    let levels = web_lib::game_level_btns().length();
    assert!(levels == 3);

    {
        let document = web_lib::document();
        let domris = domris.clone();

        let closure = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if let Some(control) = key2control(&e) {
                domris.borrow_mut().enqueue_control(control);
                e.prevent_default();
            }
        }) as Box<dyn FnMut(_)>);
        document
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        for i in 0..levels {
            let domris = domris.clone();
            let game_level_btns = web_lib::game_level_btns();
            let btn_clicked = web_lib::game_level_btn(&game_level_btns, i);

            let closure = Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
                if domris.borrow().playing() {
                    return;
                }
                for j in 0..levels {
                    let level_btn = web_lib::game_level_btn(&game_level_btns, j);
                    if i == j {
                        level_btn.class_list().add_1("active").unwrap();
                    } else {
                        level_btn.class_list().remove_1("active").unwrap();
                    }
                }
                domris.borrow_mut().set_level(i);
            }) as Box<dyn FnMut(_)>);
            btn_clicked
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
    }
    {
        // これ何とかならないのか？
        let domris_clone = domris.clone();
        let mut timer = Timer::new();
        let ctx = web_lib::ctx();
        let point = web_lib::point();
        let game_level_btns = web_lib::game_level_btns();
        let game_toggle_btn = web_lib::game_toggle_btn();
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let result = domris_clone.borrow_mut().update(timer.tick() as u32);
            drawer::draw(&domris_clone.borrow(), &ctx, result);
            if result {
                point.set_inner_text(&domris_clone.borrow().point.to_string());
            }
            let request_id = web_lib::request_animation_frame(f.borrow().as_ref().unwrap());
            if domris_clone.borrow().playing() {
                return;
            }
            game_toggle_btn.set_inner_text("Play!");
            game_toggle_btn.class_list().remove_1("disabled").unwrap();
            for i in 0..levels {
                let game_level_btn = web_lib::game_level_btn(&game_level_btns, i);
                game_level_btn.class_list().remove_1("disabled").unwrap();
            }
            web_lib::cancel_animation_frame(request_id);
        }) as Box<dyn FnMut()>));

        let domris = domris.clone();
        let ctx = web_lib::ctx();
        let game_level_btns = web_lib::game_level_btns();
        let game_toggle_btn = web_lib::game_toggle_btn();

        let closure = Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
            if domris.borrow().playing() {
                return;
            }
            web_lib::request_animation_frame(g.borrow().as_ref().unwrap());

            game_toggle_btn.set_inner_text("Playing");
            game_toggle_btn.class_list().add_1("disabled").unwrap();
            for i in 0..levels {
                if i == domris.borrow().level {
                    continue;
                }
                let level_btn = web_lib::game_level_btn(&game_level_btns, i);
                level_btn.class_list().add_1("disabled").unwrap();
            }
            domris.borrow_mut().start();
            drawer::draw(&domris.borrow(), &ctx, true);
        }) as Box<dyn FnMut(_)>);

        web_lib::game_toggle_btn()
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
}
