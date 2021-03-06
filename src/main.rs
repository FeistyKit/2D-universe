mod bodies;
mod gui;
mod shapes;
#[path = "./buttons/sliders.rs"]
mod sliders;
#[path = "./buttons/testbutton.rs"]
mod testbutton;
mod trails;
mod widgets;

use bodies::WorldSpace;
use sfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow},
    system::Vector2,
    window::{mouse::Button, Event, Key, Style},
};
use std::f32::consts::PI;
const CONSOLAS_BYTES: &[u8] = include_bytes!("assets/Consolas.ttf");
pub const WINDOW_SIZE: (f32, f32) = (1600.0, 1600.0);

use crate::gui::Gui;
fn main() {
    let consolas = Font::from_memory(CONSOLAS_BYTES).unwrap();
    let mut space = WorldSpace::deserialize("space.json").unwrap_or_default();
    space.focused_idx = Some(0);
    let mut window = RenderWindow::new(
        (WINDOW_SIZE.0 as u32, WINDOW_SIZE.1 as u32),
        "Universe simulator",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(45);

    let mut gui = Gui::new(window.size(), &consolas);
    'running: while window.is_open() {
        while let Some(event) = window.poll_event() {
            if handle_events(event, &mut window, &mut space, &mut gui) {
                space.serialize("space.json").unwrap();
                break 'running;
            }
        }
        window.set_active(true);
        window.clear(Color::BLACK);
        space.advance(&mut window, &Default::default());
        gui.update_draw(&mut window);
        gui.update_draw_focused_display(space.prepare_for_gui(), &mut window);
        window.display();
    }
}

fn handle_events<'a, 'b: 'a>(
    event: Event,
    window: &mut RenderWindow,
    space: &'a mut WorldSpace<'b>,
    gui: &'a mut Gui<'b>,
) -> bool {
    if event == Event::Closed {
        window.close();
    } else if let Event::KeyPressed {
        code,
        alt: _,
        ctrl,
        shift: _,
        system: _,
    } = event
    {
        if code == Key::S && ctrl {
            window.close();
            return true;
        } else if code == Key::F {
            space.switch_stopped();
        } else if code == Key::G {
            println!("You found my dev key!");
            println!("{:?}", gui.widgets);
        } else if code == Key::Right {
            space.advance_focused_idx();
        } else if code == Key::Up {
            gui.increase_example_mass();
        } else if code == Key::Down {
            gui.decrease_example_mass();
        } else if code == Key::C {
            space.clear_bodies();
        } else if code == Key::Left {
            space.reduce_focused_index();
        } else if code == Key::Delete || code == Key::BackSpace {
            space.remove_selected();
        }
    } else if let Event::MouseButtonPressed { button, x, y } = event {
        if button == Button::Left {
            gui.click(space, Vector2::new(x, y));
        }
    } else if let Event::MouseButtonReleased { button, x: _, y: _ } = event {
        if button == Button::Left {
            gui.release_click(space);
        }
    } else if let Event::MouseMoved { x, y } = event {
        gui.mouse_moved(x, y);
    }
    false
}
