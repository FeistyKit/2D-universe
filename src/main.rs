mod bodies;

use bodies::{SpaceBody, WorldSpace};
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{Event, Style},
};
use std::f32::consts::PI;
fn main() {
    let p1 = SpaceBody::new(90.0, 90.0, 1000.0, 50.0, 0.0, 10.0, false);
    let p2 = SpaceBody::new(500.0, 500.0, 500.0, 25.0, 0.0, 10.0, false);
    let mut space = WorldSpace::with_bodies(vec![p1, p2]);
    let mut window = RenderWindow::new(
        (1600, 1600),
        "Universe simulator",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(45);
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }
        window.set_active(true);
        window.clear(Color::BLACK);
        space.update_acceleration();
        space.update_positions();
        space.update_time();
        space.draw(&mut window, &Default::default());
        window.display();
    }
}
