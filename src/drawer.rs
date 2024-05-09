use std::cmp::min;
use std::time::SystemTime;
use chrono::{DateTime, Local, Timelike};
use sfml::graphics::{Color, Font, RenderTarget, RenderWindow, Text, Transformable};
use sfml::SfBox;
use sfml::system::{Vector2f};

fn clock(font: &SfBox<Font>, view_size: Vector2f) -> Text {
    let now = SystemTime::now();
    let dt: DateTime<Local> = now.clone().into();
    let time_text = format!("{:0>2}:{:0>2}:{:0>2}", dt.hour(), dt.minute(), dt.second());

    let mut text = Text::new(
        time_text.as_str(),
        &font,
        min(view_size.x as u32, view_size.y as u32) / 12
    );

    text.set_fill_color(Color::WHITE);
    let local_bounds = text.local_bounds();
    text.set_origin(
        Vector2f::new(
            local_bounds.left + local_bounds.width / 2f32,
            local_bounds.top + local_bounds.height / 2f32
        )
    );

    text.set_position(
        Vector2f::new(
            view_size.x / 2f32, view_size.y / 2f32
        )
    );

    return text
}

pub struct Drawer {
    clock_font: SfBox<Font>
}

impl Drawer {
    pub fn new() -> Drawer {
        let font = Font::from_file("/usr/share/fonts/TTF/RobotoMono-Regular.ttf").unwrap();
        return Drawer {
            clock_font: font
        }
    }

    pub fn draw(&self, window: &mut RenderWindow) {
        window.clear(Color::BLACK);
        window.clear(Color::rgb(0, 137, 156));

        let text = clock(&self.clock_font, window.view().size());
        window.draw(&text);
        window.display();
    }
}