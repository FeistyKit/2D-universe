use std::{
    cell::RefCell,
    collections::BTreeSet,
    convert::TryInto,
    ops::{Deref, DerefMut},
};

use sfml::{
    graphics::{
        CircleShape, Color, Drawable, Font, RenderTarget, RenderWindow, Shape, Text, Transformable,
    },
    system::{SfBox, Vector2, Vector2f},
};

use crate::{
    bodies::{SpaceBody, WorldSpace},
    sliders::{ColorType, Slider},
    widgets::Widget,
    WINDOW_SIZE,
};
const NEW_PLANET_SPEED_MOD: f32 = 2.5;
#[derive(Debug)]
pub struct Gui<'a> {
    example_planet: CircleShape<'a>,
    held_position: Option<Vector2f>,
    radius: f32,
    size: Vector2<u32>,
    font: &'a SfBox<Font>,
    mass: f32,
    text: Option<Text<'a>>,
    trail_line: Option<[GuideLinePoint<'a>; 10]>,
    focused_planet: Option<CircleShape<'a>>,
    focused_number_display: Option<Text<'a>>,
    pub widgets: BTreeSet<RefCell<Box<dyn Widget>>>,
    click_held: Option<usize>,
}

impl<'a> Gui<'a> {
    pub fn new(size: Vector2<u32>, font: &'a SfBox<Font>) -> Gui<'a> {
        let mut circle = CircleShape::new(30.0, 100);
        let default_radius = 30;
        circle.set_position((
            (default_radius) as f32,
            (size.y - 2 * default_radius) as f32,
        ));
        #[allow(clippy::mutable_key_type)]
        let mut set = BTreeSet::new();
        set.insert(RefCell::new(Box::new(Slider::new(
            ColorType::Blue,
            set.len(),
            (10.0, 1400.0),
            (255.0, 20.0),
            5.0,
        )) as Box<dyn Widget>));
        set.insert(RefCell::new(Box::new(Slider::new(
            ColorType::Green,
            set.len(),
            (10.0, 1300.0),
            (255.0, 20.0),
            5.0,
        )) as Box<dyn Widget>));
        set.insert(RefCell::new(Box::new(Slider::new(
            ColorType::Red,
            set.len(),
            (10.0, 1200.0),
            (255.0, 20.0),
            5.0,
        )) as Box<dyn Widget>));

        Gui {
            example_planet: circle,
            held_position: None,
            radius: 30.0,
            size,
            font,
            mass: 30.0,
            text: None,
            trail_line: None,
            focused_planet: None,
            focused_number_display: None,
            widgets: set,
            click_held: None,
        }
    }
    pub fn mouse_moved(&mut self, x: i32, y: i32) {
        if let Some(wid) = self.click_held {
            self.widgets
                .iter()
                .nth(wid)
                .unwrap()
                .borrow_mut()
                .mouse_moved(&mut self.example_planet, x, y);
        }
    }
    pub fn update_draw(&mut self, target: &mut RenderWindow) {
        if self.held_position.is_none() {
            self.example_planet.draw(target, Default::default());
        }
        if self.text.is_none() {
            self.text = Some(Text::new(&self.mass.to_string(), &self.font, 30));
        }
        if self.text.as_ref().unwrap().string().to_rust_string() != self.mass.to_string() {
            self.text
                .as_mut()
                .unwrap()
                .set_string(&self.mass.to_string());
        }
        self.text.as_ref().unwrap().draw(target, Default::default());
        assert_eq!(self.held_position.is_none(), self.trail_line.is_none());
        if self.held_position.is_some() {
            self.update_guideline(target.mouse_position());
            self.draw_guideline(target);
        }
        self.draw_widgets(target);
    }
    fn draw_widgets(&self, target: &mut dyn RenderTarget) {
        for widget in &self.widgets {
            widget.borrow().draw(target);
        }
    }
    pub fn click(&mut self, space: &mut WorldSpace, mouse_pos: Vector2<i32>) {
        for widget in self.widgets.iter() {
            let bounds = widget.borrow().get_bounds();
            if inside(bounds, mouse_pos) {
                widget.borrow_mut().click(self, space);
                self.click_held = Some(widget.borrow().get_layer());
                return;
            }
        }
        let adj_pos_x = mouse_pos.x as f32;
        let adj_pos_y = mouse_pos.y as f32;
        if self.held_position.is_some() {
            space.push_body(SpaceBody::new(
                (
                    adj_pos_x + space.cam_pos.x - WINDOW_SIZE.0 / 2.0,
                    adj_pos_y + space.cam_pos.y - WINDOW_SIZE.1 / 2.0,
                ),
                self.mass,
                self.radius,
                (mouse_pos.x as f32 - self.held_position.unwrap().x) / NEW_PLANET_SPEED_MOD,
                (mouse_pos.y as f32 - self.held_position.unwrap().y) / NEW_PLANET_SPEED_MOD,
                false,
                self.example_planet.fill_color(),
                space.bodies.len(),
            ));
            self.held_position = None;
            self.trail_line = None;
        } else {
            self.held_position = Some(Vector2f::new(adj_pos_x, adj_pos_y));
            self.update_guideline(mouse_pos);
        }
    }
    pub fn release_click(&mut self, space: &mut WorldSpace) {
        self.click_held = None;
        for widget in self.widgets.iter() {
            if widget.borrow().is_click_held() {
                widget
                    .borrow_mut()
                    .release_click(&mut self.example_planet, space);
            }
        }
    }
    fn update_guideline(&mut self, mouse_pos: Vector2<i32>) {
        let adj_pos_x = mouse_pos.x as f32;
        let adj_pos_y = mouse_pos.y as f32;
        self.trail_line = Some(
            (0..10)
                .into_iter()
                .map(|i| {
                    let p = i as f32;
                    let mut circle = CircleShape::new(5.0, 20);
                    circle.set_fill_color(Color::rgb(120, 125, 129));
                    circle.set_position(Vector2f::new(
                        -(self.held_position.unwrap().x - adj_pos_x) * p / 10.0
                            + self.held_position.unwrap().x,
                        -(self.held_position.unwrap().y - adj_pos_y) * p / 10.0
                            + self.held_position.unwrap().y,
                    ));
                    GuideLinePoint { circle }
                })
                .collect::<Vec<GuideLinePoint>>()
                .try_into()
                .unwrap(),
        );
    }
    fn draw_guideline(&self, target: &mut dyn RenderTarget) {
        for i in self.trail_line.as_ref().unwrap().iter() {
            i.draw(target, Default::default());
        }
    }
    pub fn update_draw_focused_display(
        &mut self,
        opt: Option<(CircleShape<'a>, usize)>,
        target: &mut dyn RenderTarget,
    ) {
        if let Some(pair) = opt {
            let mut shape = pair.0;
            let index = pair.1;
            shape.set_position((WINDOW_SIZE.0 - shape.radius() * 2.0 - 30.0, 0.0));
            let mut text = Text::new(&format!("#{}", index + 1), self.font, 50);
            text.set_position((
                WINDOW_SIZE.0 - 3.0 * shape.radius(),
                shape.radius() * 2.0 + 30.0,
            ));
            self.focused_number_display = Some(text);
            self.focused_planet = Some(shape);
            target.draw(self.focused_number_display.as_ref().unwrap());
            target.draw(self.focused_planet.as_ref().unwrap());
        } else {
            self.focused_planet = None;
            self.focused_number_display = {
                let mut text = Text::new("No planet selected.", self.font, 30);
                text.set_position((WINDOW_SIZE.0 - 350.0, 0.0));
                text.set_fill_color(Color::WHITE);
                Some(text)
            };
            target.draw(self.focused_number_display.as_ref().unwrap());
        }
    }
    pub fn increase_example_mass(&mut self) {
        self.mass += 5.0;
    }
    pub fn decrease_example_mass(&mut self) {
        self.mass -= 5.0;
    }
}
#[derive(Debug)]
struct GuideLinePoint<'a> {
    circle: CircleShape<'a>,
}
impl<'a> Deref for GuideLinePoint<'a> {
    type Target = CircleShape<'a>;

    fn deref(&self) -> &Self::Target {
        &self.circle
    }
}
impl<'a> DerefMut for GuideLinePoint<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.circle
    }
}

fn inside(a: (Vector2f, Vector2f), b: Vector2<i32>) -> bool {
    a.0.x < b.x as f32 && a.0.y < b.y as f32 && a.1.x > b.x as f32 && a.1.y > b.y as f32
}
