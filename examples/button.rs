#![no_std]
#![no_main]

use punda::{
    button::{Button, State},
    serial,
};

punda::punda!(init = init, button_handler = button_handler);

fn init(cx: &mut punda::context::UserContext) {
    serial::println(cx, "Press a button".into());
}

fn button_handler(cx: &mut punda::context::UserContext, button: Button, direction: State) {
    let message = match (button, direction) {
        (Button::A, State::Released) => "A released",
        (Button::B, State::Released) => "B released",
        (Button::A, State::Pushed) => "A pushed",
        (Button::B, State::Pushed) => "B pushed",
    };

    serial::println(cx, message.into());
}
