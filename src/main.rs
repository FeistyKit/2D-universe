mod bodies;
mod gui;
mod trails;

use bodies::WorldSpace;
use sfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow},
    window::{Event, Key, Style},
    SfBox,
};
use std::{error::Error, f32::consts::PI, fs};
const CONSOLAS_BYTES: &[u8] = include_bytes!("assets/Consolas.ttf");

use crate::gui::Gui;
fn main() {
    let mut space = WorldSpace::deserialize("space.json").unwrap_or_default();
    let mut window = RenderWindow::new(
        (1600, 1600),
        "Universe simulator",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(45);
    let gui = Gui::new(
        window.size(),
        font_from_file().unwrap_or_else(|_| Font::from_memory(CONSOLAS_BYTES).unwrap()),
    );
    'running: while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
            if let Event::KeyPressed {
                code,
                alt: _,
                ctrl: _,
                shift: _,
                system: _,
            } = event
            {
                if code == Key::S {
                    window.close();
                    space.serialize("space.json").unwrap();
                    break 'running;
                } else if code == Key::A {
                    println!("{:?}", window.mouse_position());
                } else if code == Key::F {
                    space.switch_stopped();
                } else if code == Key::G {
                    println!("{:?}", space);
                }
            }
        }
        window.set_active(true);
        window.clear(Color::BLACK);

        assert!(gui.held_position.is_none());
        space.advance(&mut window, &Default::default());
        gui.draw(&mut window);
        window.display();
    }
}
pub fn font_from_file() -> Result<SfBox<Font>, Box<dyn Error>> {
    for file in fs::read_dir("./")? {
        let os_name = file?.file_name();
        let name = os_name.to_string_lossy();
        if name.ends_with(".ttf") {
            if let Some(font) = Font::from_file(&name) {
                return Ok(font);
            }
        }
    }
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "not found",
    )))
}
