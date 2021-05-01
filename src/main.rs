mod bodies;
mod trails;

use bodies::{SpaceBody, WorldSpace};
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{Event, Key, Style},
};
use std::f32::consts::PI;
fn main() {
    let mut space = WorldSpace::deserialize("space.json").unwrap_or_default();
    let mut window = RenderWindow::new(
        (1600, 1600),
        "Universe simulator",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(45);
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
        space.advance(&mut window, &Default::default());
        window.display();
    }
}
