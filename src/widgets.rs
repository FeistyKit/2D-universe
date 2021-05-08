use sfml::{
    graphics::{CircleShape, Color, Drawable, RectangleShape, RenderTarget, Shape, Transformable},
    system::Vector2f,
};
use std::{f32::consts::PI, fmt::Debug, usize};

use crate::{bodies::WorldSpace, gui::Gui};
#[derive(Debug)]
pub enum WidgetKind {
    RoundedRect,
}

pub trait Widget {
    fn get_bounds(&self) -> (Vector2f, Vector2f);
    fn get_layer(&self) -> usize;
    fn draw(&self, target: &mut dyn RenderTarget);
    fn widget_type(&self) -> WidgetKind;
    fn click(&mut self, gui: &mut Gui, space: &mut WorldSpace);
    fn release_click(&mut self, gui: &mut Gui, space: &mut WorldSpace);
}
impl PartialOrd for dyn Widget {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_layer().partial_cmp(&other.get_layer())
    }
}
impl PartialEq for dyn Widget {
    fn eq(&self, other: &Self) -> bool {
        self.get_layer() == other.get_layer()
    }
}
impl Debug for dyn Widget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{{layer: {}, bounds: {:?}}}",
            self.widget_type(),
            self.get_layer(),
            self.get_bounds()
        )
    }
}
impl Ord for dyn Widget {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_layer().cmp(&other.get_layer())
    }
}
impl Eq for dyn Widget {}
pub struct TestButton<'a> {
    layer: usize,
    rect: RoundedRect<'a>,
}
pub struct RoundedRect<'a> {
    radius: f32,
    circles: [CircleShape<'a>; 4], //circles are in order like quadrants of coordinate grid
    rectangles: [RectangleShape<'a>; 2],
    dimensions: Vector2f,
    position: Vector2f,
    color: Color,
}
impl RoundedRect<'_> {
    #[allow(unused)]
    pub fn new<'a, T>(radius: f32, bdimensions: T, bposition: T, color: Color) -> RoundedRect<'a>
    where
        T: Into<Vector2f>,
    {
        let dimensions: Vector2f = bdimensions.into();
        let position: Vector2f = bposition.into();
        assert!(
            radius * 2.0 <= dimensions.x,
            "The radius was too large for a RoundedRect of x size {}",
            dimensions.x
        );
        assert!(
            radius * 2.0 <= dimensions.y,
            "The radius was too large for a RoundedRect of x size {}",
            dimensions.y
        );
        let mut def = CircleShape::new(radius, (radius * PI).ceil() as u32);
        def.set_fill_color(color);
        let mut top_left = def.clone();
        top_left.set_position(position);
        let mut top_right = def.clone();
        top_right.set_position((position.x + dimensions.x - radius * 2.0, position.y));
        let mut bottom_left = def.clone();
        bottom_left.set_position((position.x, position.y + dimensions.y - 2.0 * radius));
        let mut bottom_right = def.clone();
        bottom_right.set_position((
            position.x + dimensions.x - radius * 2.0,
            position.y + dimensions.y - radius * 2.0,
        ));
        drop(def);
        let circles = [top_right, top_left, bottom_left, bottom_right];
        let mut up_down =
            RectangleShape::with_size(Vector2f::new(dimensions.x - radius * 2.0, dimensions.y));
        up_down.set_fill_color(color);
        up_down.set_position((position.x + radius, position.y));
        let mut left_right =
            RectangleShape::with_size(Vector2f::new(dimensions.x, dimensions.y - 2.0 * radius));
        left_right.set_fill_color(color);
        left_right.set_position((position.x, position.y + radius));
        RoundedRect {
            radius,
            circles,
            rectangles: [up_down, left_right],
            dimensions,
            position,
            color,
        }
    }
    #[allow(unused)]
    pub fn set_position<P: Into<Vector2f>>(&mut self, pos: P) {
        todo!()
    }
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
    pub fn set_colour(&mut self, color: Color) {
        for circle in &mut self.circles {
            circle.set_fill_color(color);
        }
        for rect in &mut self.rectangles {
            rect.set_fill_color(color);
        }
    }
    pub fn get_colour(&self) -> Color {
        self.color
    }
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
        WidgetKind::RoundedRect
    }
    #[allow(unused)]
    fn click(&mut self, gui: &mut Gui, space: &mut WorldSpace) {
        todo!()
    }
    #[allow(unused)]
    fn release_click(&mut self, gui: &mut Gui, space: &mut WorldSpace) {
        todo!()
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
        }
    }
}
