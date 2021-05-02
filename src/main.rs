mod bodies;
mod gui;
mod trails;

use bodies::WorldSpace;
use sfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow},
    system::Vector2,
    window::{mouse::Button, Event, Key, Style},
};
use std::f32::consts::PI;
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
    let consolas = Font::from_memory(CONSOLAS_BYTES).unwrap();
    let mut gui = Gui::new(window.size(), &consolas);
    'running: while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            } else if let Event::KeyPressed {
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
            } else if let Event::MouseButtonPressed { button, x, y } = event {
                if button == Button::LEFT {
                    gui.click(&mut space, Vector2::new(x, y));
                }
            }
        }
        window.set_active(true);
        window.clear(Color::BLACK);
        space.advance(&mut window, &Default::default());
        gui.update_draw(&mut window);
        window.display();
    }
}
