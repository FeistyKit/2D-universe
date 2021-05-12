use sfml::{
    graphics::{CircleShape, Color, RenderTarget, Transformable},
    system::Vector2f,
};
use std::usize;

use crate::{
    bodies::WorldSpace,
    gui::Gui,
    shapes::RoundedRect,
    widgets::{Widget, WidgetKind},
};
#[derive(Debug)]
pub struct TestButton<'a> {
    layer: usize,
    rect: RoundedRect<'a>,
    is_click_held: bool,
}
impl Widget for TestButton<'_> {
    fn get_bounds(&self) -> (Vector2f, Vector2f) {
        (
            self.rect.position,
            self.rect.position + self.rect.dimensions,
        )
    }

    fn get_layer(&self) -> usize {
        self.layer
    }

    fn draw(&self, target: &mut dyn RenderTarget) {
        self.rect.draw(target);
    }

    fn widget_type(&self) -> WidgetKind {
        WidgetKind::TestButton
    }
    #[allow(unused)]
    fn click(&mut self, gui: &Gui, space: &mut WorldSpace) {
        self.rect.set_fill_color(Color::RED);
        self.is_click_held = true;
    }
    #[allow(unused)]
    fn release_click(&mut self, gui: &mut CircleShape, space: &mut WorldSpace) {
        self.rect.set_fill_color(Color::BLUE);
        self.is_click_held = false;
    }

    fn is_click_held(&self) -> bool {
        self.is_click_held
    }
    fn debug_string(&self) -> std::string::String {
        format!("{:?}", self)
    }
    fn mouse_moved(&mut self, _: &mut CircleShape, x: i32, y: i32) {
        self.rect.set_origin((x as f32, y as f32));
    }
}
#[allow(unused)]
impl TestButton<'_> {
    pub fn new<'a, T>(
        layer: usize,
        radius: f32,
        dimensions: T,
        position: T,
        color: Color,
    ) -> TestButton<'a>
    where
        T: Into<Vector2f>,
    {
        TestButton {
            layer,
            rect: RoundedRect::new(radius, dimensions, position, color),
            is_click_held: false,
        }
    }
    pub fn default(layer: usize) -> Self {
        TestButton::new(layer, 10.0, (400.0, 400.0), (200.0, 200.0), Color::BLUE)
    }
}
