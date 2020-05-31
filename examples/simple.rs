#![no_std]
#![no_main]

punda::punda!(init = init);

fn init(_cx: &mut punda::context::InitContext) {}
