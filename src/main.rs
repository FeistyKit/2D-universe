mod bodies;
mod gui;
mod trails;

use bodies::WorldSpace;
use sfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow},
    system::{Vector2, Vector2f},
    window::{mouse::Button, Event, Key, Style},
};
use std::f32::consts::PI;
const CONSOLAS_BYTES: &[u8] = include_bytes!("assets/Consolas.ttf");
pub const WINDOW_SIZE: (f32, f32) = (1600.0, 1600.0);

use crate::gui::Gui;
fn main() {
    let mut space = WorldSpace::deserialize("space.json").unwrap_or_default();
    space.focused_idx = Some(0);
    let mut window = RenderWindow::new(
        (WINDOW_SIZE.0 as u32, WINDOW_SIZE.1 as u32),
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
                    println!(
                        "{:?}, {:?}, {:?}, {:?}",
                        {
                            let pos = window.mouse_position();
                            let x = pos.x as f32 + space.cam_pos.x;
                            let y = pos.y as f32 + space.cam_pos.y;
                            (x, y)
                        },
                        (
                            space.bodies[space.focused_idx.unwrap()].x,
                            space.bodies[space.focused_idx.unwrap()].y
                        ),
                        space.cam_pos,
                        (
                            space.bodies[space.focused_idx.unwrap()].x + space.cam_pos.x,
                            space.bodies[space.focused_idx.unwrap()].y + space.cam_pos.y
                        )
                    );
                } else if code == Key::F {
                    space.switch_stopped();
                } else if code == Key::G {
                    space.focused_idx = Some(0);
                    println!("G was pressed, {:?}", space.cam_pos);
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
