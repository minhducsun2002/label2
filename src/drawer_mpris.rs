use std::cmp::min;
use chrono::{Timelike, TimeZone, Utc};
use sfml::graphics::{Color, Font, Text, Transformable};
use sfml::SfBox;
use sfml::system::Vector2f;

use crate::mpris::PlayState;

fn get_text<'a, 'b>(font: &'a SfBox<Font>, content: &'b str, view_size: Vector2f<>) -> Text<'a> {
    let text = Text::new(content, &font, min(view_size.x as u32, view_size.y as u32) / 30);

    return text;
}

fn song(font: &SfBox<Font>, state: PlayState, view_size: Vector2f) -> Text {
    let artist: String = state.artist.join(", ");
    let title = state.title;

    let text = format!("{}  —  {}", artist, title);

    let mut text = get_text(font, text.as_str(), view_size);

    text.set_fill_color(Color::rgba(255, 255, 255, if state.playing {255} else {100}));
    let local_bounds = text.local_bounds();
    text.set_origin(
        Vector2f::new(
            local_bounds.left + local_bounds.width / 2f32,
            local_bounds.top + local_bounds.height / 2f32
        )
    );

    text.set_position(
        Vector2f::new(
            view_size.x / 2f32,
            view_size.y - (local_bounds.height / 2f32) - 20f32
        )
    );

    return text;
}

fn time(font: &SfBox<Font>, seconds: u32, view_size: Vector2f, is_start: bool) -> Text {
    let dt = Utc.timestamp_opt(seconds as i64, 0).unwrap();
    let time_text;

    if dt.hour() > 0 {
        time_text = format!("{:0>2}:{:0>2}:{:0>2}", dt.hour(), dt.minute(), dt.second());
    } else {
        time_text = format!("{:0>2}:{:0>2}", dt.minute(), dt.second());
    }

    let mut text = get_text(font, time_text.as_str(), view_size);

    text.set_fill_color(Color::rgba(110, 238, 255, 80));
    let local_bounds = text.local_bounds();
    text.set_origin(
        Vector2f::new(
            local_bounds.left + local_bounds.width / 2f32,
            local_bounds.top + local_bounds.height / 2f32
        )
    );

    if is_start {
        text.set_position(
            Vector2f::new(
                local_bounds.width / 2f32 + 28f32,
                view_size.y - (local_bounds.height / 2f32) - 28f32
            )
        );
    } else {
        text.set_position(
            Vector2f::new(
                view_size.x - (local_bounds.width / 2f32) - 26f32,
                view_size.y - (local_bounds.height / 2f32) - 26f32
            )
        );
    }

    return text;
}

// artist - song, duration elapsed, total
pub fn music(font: &SfBox<Font>, state: PlayState, view_size: Vector2f) -> (Text, Option<Text>, Option<Text>) {
    let song_text = song(font, state.clone(), view_size);

    if state.seconds_total != 0 {
        let start = time(font, state.seconds_played, view_size, true);
        let end = time(font, state.seconds_total, view_size, false);
        return (song_text, Some(start), Some(end));
    }

    return (song_text, None, None);
}