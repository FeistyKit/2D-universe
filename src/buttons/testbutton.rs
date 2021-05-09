use sfml::{
    graphics::{Color, Drawable, RenderTarget},
    system::Vector2f,
};
use std::usize;

use crate::{
    bodies::WorldSpace,
    gui::Gui,
    shapes::RoundedRect,
    widgets::{Widget, WidgetKind},
};

pub struct TestButton<'a> {
    layer: usize,
    rect: RoundedRect<'a>,
    has_been_clicked: bool,
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
        self.rect
            .circles
            .iter()
            .for_each(|f| f.draw(target, Default::default()));
        self.rect
            .rectangles
            .iter()
            .for_each(|r| r.draw(target, Default::default()));
    }

    fn widget_type(&self) -> WidgetKind {
        WidgetKind::TestButton
    }
    #[allow(unused)]
    fn click(&mut self, gui: &Gui, space: &mut WorldSpace) {
        self.rect.set_fill_color(Color::TRANSPARENT);
        self.has_been_clicked = true;
    }
    #[allow(unused)]
    fn release_click(&mut self, gui: &Gui, space: &mut WorldSpace) {
        self.rect.set_fill_color(Color::BLUE);
        self.has_been_clicked = false;
    }

    fn has_been_clicked(&self) -> bool {
        self.has_been_clicked
    }
}
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
            has_been_clicked: false,
        }
    }
    pub fn default(layer: usize) -> Self {
        TestButton::new(layer, 10.0, (400.0, 400.0), (200.0, 200.0), Color::BLUE)
    }
}
