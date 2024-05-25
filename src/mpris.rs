use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use mpris::{PlaybackStatus, PlayerFinder};

#[derive(Clone)]
pub struct PlayState {
    pub playing: bool,
    pub title: String,
    pub artist: Vec<String>,
    pub seconds_total: u32,
    pub seconds_played: u32
}

pub fn mpris(sender: Sender<Option<PlayState>>) {
    let player_finder = PlayerFinder::new();
    if let Err(e) = player_finder {
        eprintln!("{:?}", e);
        return;
    }

    let finder = player_finder.unwrap();
    loop {
        thread::sleep(Duration::from_millis(500));
        if let Ok(active) = finder.find_active() {
            if let (Ok(status), Ok(metadata)) = (active.get_playback_status(), active.get_metadata()) {
                let title = metadata.title().unwrap_or("");
                let artist = metadata.artists().unwrap_or(Vec::new());

                let mut state = PlayState {
                    playing: status == PlaybackStatus::Playing,
                    title: title.to_string(),
                    artist: artist.iter().map(|s| s.to_string()).collect(),
                    seconds_total: 0,
                    seconds_played: 0
                };

                let duration = metadata.length().unwrap_or(Duration::new(0, 0)).as_secs();
                let position = active.get_position().unwrap_or(Duration::new(0, 0)).as_secs();

                if duration != 0 {
                    state.seconds_total = duration as u32;
                    state.seconds_played = position as u32;
                }

                let _ = sender.send(Some(state));
                continue
            }
        }

        let _ = sender.send(None);
    }
}