// #![allow(unused, dead_code)]
extern crate sdl2;


use sdl2::pixels::Color;
use sdl2::event::Event;

use std::time::Duration;
use std::time::Instant;

use std::fs::read_to_string;
use rlua::{prelude::*, StdLib};

pub fn main() {
    let script_code = read_to_string("main.lua").unwrap();
    let mut lua_libs = StdLib::empty();
    lua_libs.insert(StdLib::BASE);
    let lua = Lua::new_with(lua_libs);

    let mut title = String::from("Untitled");
    let mut width: u32 = 0;
    let mut height: u32 = 0;

    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();
        lua_ctx.load(&script_code).exec().unwrap();
        let window_table = globals.get::<_, LuaTable>("window").unwrap();
        if let Some(value) = window_table.get::<_, String>("title").ok() {
            title = value;
        }
        if let Some(value) = window_table.get::<_, u32>("width").ok() {
            width = value;
        }
        if let Some(value) = window_table.get::<_, u32>("height").ok() {
            height = value;
        }

        let lua_update = globals.get::<_, LuaFunction>("on_update").ok();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem.window(&title, width, height)
            .position_centered()
            .build()
            .unwrap();
    
        let mut canvas = window.into_canvas().build().unwrap();
    
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        if let Some(item) = globals.get::<_, LuaFunction>("on_ready").ok() {
            item.call::<_, ()>(()).unwrap();
        }

        let mut last = Instant::now();

        'running: loop {
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 100));
            let now = Instant::now();
            if let Some(item) = &lua_update {
                item.call::<_, ()>((now-last).as_secs_f64()).unwrap();
            }
            last = now;

            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        if let Some(item) = globals.get::<_, LuaFunction>("on_quit").ok() {
                            if item.call::<_, bool>(()).unwrap() {
                                break 'running
                            }
                        } else {
                            break 'running
                        }
                    },
                    Event::KeyDown { keycode: Some(key), .. } => {
                        if let Some(item) = globals.get::<_, LuaFunction>("on_keydown").ok() {
                            item.call::<_, ()>(key.to_string()).unwrap()
                        }
                    },
                    _ => {}
                }
            }
            // The rest of the game loop goes here...
    
            canvas.present();
        }
    });
}