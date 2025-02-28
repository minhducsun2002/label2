mod drawer;
mod mpris;
mod drawer_mpris;

use std::{env, thread};
use std::sync::mpsc;
use sfml::graphics::{RenderWindow, FloatRect, View, RenderTarget};
use sfml::window::{ContextSettings, Event, Handle, Style};
use crate::drawer::Drawer;
use crate::mpris::mpris;

fn main() {
    let window_id = env::var("XSCREENSAVER_WINDOW");
    let mut maybe_render_window;
    let mut xscreensaver_window = 0;

    if let Ok(wid) = window_id {
        if let Ok(wid) = wid.parse::<i32>() {
            xscreensaver_window = wid;
        }
    }
    if xscreensaver_window != 0 {
        unsafe {
            maybe_render_window = RenderWindow::from_handle(xscreensaver_window as Handle, &ContextSettings::default());
        }
    } else {
        maybe_render_window = RenderWindow::new(
            (1280, 720),
            "Yep",
            Style::DEFAULT,
            &ContextSettings::default()
        );
    }
    
    let mut rw;
    
    match maybe_render_window {
        Ok(r) => {
            rw = r
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
    
    rw.set_framerate_limit(2);
    let mut drawer = Drawer::new();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        mpris(tx);
    });

    let mut frame = 2;

    while rw.is_open() {
        while let Some(ev) = rw.poll_event() {
            match ev {
                Event::Closed => rw.close(),
                Event::Resized { width, height } => {
                    let rect = FloatRect::new(0f32, 0f32, width as f32, height as f32);
                    let view = View::from_rect(rect).unwrap(); // so this can blow up?
                    rw.set_view(&view);
                },
                _ => {}
            }
        }

        if let Ok(result) = rx.recv() {
            if result.is_none() {
                if frame != 2 {
                    rw.set_framerate_limit(2);
                    frame = 2;
                    println!("Framerate {}", frame);
                }
            } else {
                if frame != 15 {
                    rw.set_framerate_limit(15);
                    frame = 15;
                    println!("Framerate {}", frame);
                }
            }
            drawer.music_state = result;
        }


        drawer.draw(&mut rw);
    }
}