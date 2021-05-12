use sfml::{
    graphics::{
        CircleShape, Color, Drawable, PrimitiveType, RenderTarget, Shape, Transformable, Vertex,
        VertexArray,
    },
    system::Vector2f,
};

use crate::{
    bodies::WorldSpace,
    gui::Gui,
    shapes::RoundedRect,
    widgets::{Widget, WidgetKind},
};
#[allow(unused)]
#[derive(Debug)]
pub enum ColorType {
    Red,
    Green,
    Blue,
    Alpha,
}
#[derive(Debug)]
pub struct Slider<'a> {
    pub handle: RoundedRect<'a>,
    pub array: VertexArray,
    pub position: Vector2f,
    pub value: u8,
    pub max_width: f32,
    layer: usize,
    clicked: bool,
    color_type: ColorType,
}

impl<'a> Slider<'a> {
    pub fn new<T: Into<Vector2f>>(
        color_type: ColorType,
        layer: usize,
        pos: T,
        dims: T,
        radii: f32,
    ) -> Slider<'a> {
        let bpos: Vector2f = pos.into();
        let bdims: Vector2f = dims.into();
        let mut array = VertexArray::new(PrimitiveType::TriangleStrip, 6);
        let vertices;
        let max_width = bdims.x;
        match color_type {
            ColorType::Red => {
                vertices = [
                    Vertex::new(bpos, Color::RED, Default::default()),
                    Vertex::new((bpos.x, bpos.y + bdims.y), Color::RED, Default::default()),
                    Vertex::new(
                        (bpos.x + max_width / 2.0, bpos.y),
                        Color::rgb(255, 127, 127),
                        Default::default(),
                    ),
                    Vertex::new(
                        (bpos.x + max_width / 2.0, bpos.y + bdims.y),
                        Color::rgb(255, 127, 127),
                        Default::default(),
                    ),
                    Vertex::new(
                        (bpos.x + max_width, bpos.y),
                        Color::WHITE,
                        Default::default(),
                    ),
                    Vertex::new(
                        (bpos.x + max_width, bpos.y + bdims.y),
                        Color::WHITE,
                        Default::default(),
                    ),
                ]
            }
            ColorType::Green => {
                vertices = [
                    Vertex::new(bpos, Color::GREEN, Default::default()),
                    Vertex::new((bpos.x, bpos.y + bdims.y), Color::GREEN, Default::default()),
                    Vertex::new(
                        (bpos.x + max_width / 2.0, bpos.y),
                        Color::rgb(127, 255, 127),
                        Default::default(),
                    ),
                    Vertex::new(
                        (bpos.x + max_width / 2.0, bpos.y + bdims.y),
                        Color::rgb(127, 255, 127),
                        Default::default(),
                    ),
                    Vertex::new(
                        (bpos.x + max_width, bpos.y),
                        Color::WHITE,
                        Default::default(),
                    ),
                    Vertex::new(
                        (bpos.x + max_width, bpos.y + bdims.y),
                        Color::WHITE,
                        Default::default(),
                    ),
                ]
            }
            ColorType::Blue => {
                vertices = [
                    Vertex::new(bpos, Color::BLUE, Default::default()),
                    Vertex::new((bpos.x, bpos.y + bdims.y), Color::BLUE, Default::default()),
                    Vertex::new(
                        (bpos.x + max_width / 2.0, bpos.y),
                        Color::rgb(127, 127, 255),
                        Default::default(),
                    ),
                    Vertex::new(
                        (bpos.x + max_width / 2.0, bpos.y + bdims.y),
                        Color::rgb(127, 127, 255),
                        Default::default(),
                    ),
                    Vertex::new(
                        (bpos.x + max_width, bpos.y),
                        Color::WHITE,
                        Default::default(),
                    ),
                    Vertex::new(
                        (bpos.x + max_width, bpos.y + bdims.y),
                        Color::WHITE,
                        Default::default(),
                    ),
                ]
            }
            ColorType::Alpha => {
                vertices = [
                    Vertex::new(bpos, Color::WHITE, Default::default()),
                    Vertex::new((bpos.x, bpos.y + bdims.y), Color::WHITE, Default::default()),
                    Vertex::new(
                        (bpos.x + max_width / 2.0, bpos.y),
                        Color::rgba(255, 255, 255, 127),
                        Default::default(),
                    ),
                    Vertex::new(
                        (bpos.x + max_width / 2.0, bpos.y + bdims.y),
                        Color::rgba(255, 255, 255, 127),
                        Default::default(),
                    ),
                    Vertex::new(
                        (bpos.x + max_width, bpos.y),
                        Color::rgba(255, 255, 255, 0),
                        Default::default(),
                    ),
                    Vertex::new(
                        (bpos.x + max_width, bpos.y + bdims.y),
                        Color::rgba(255, 255, 255, 0),
                        Default::default(),
                    ),
                ]
            }
        }
        for a in vertices.iter() {
            array.append(a);
        }
        Slider {
            handle: RoundedRect::new(
                radii,
                (15.0, bdims.y + 3.0),
                (bpos.x + bdims.x - 7.5, bpos.y),
                Color::rgb(169, 169, 169),
            ),
            array,
            position: bpos,
            value: 255,
            max_width: bdims.x,
            layer,
            clicked: false,
            color_type,
        }
    }
}
impl<'a> Widget for Slider<'a> {
    fn get_bounds(&self) -> (Vector2f, Vector2f) {
        self.handle.get_bounds()
    }

    fn get_layer(&self) -> usize {
        self.layer
    }

    fn draw(&self, target: &mut dyn RenderTarget) {
        self.array.draw(target, Default::default());
        self.handle.draw(target);
    }

    fn widget_type(&self) -> crate::widgets::WidgetKind {
        WidgetKind::Slider
    }

    fn click(&mut self, _: &Gui, _: &mut WorldSpace) {
        self.clicked = true;
    }

    fn release_click(&mut self, _: &mut CircleShape, _: &mut WorldSpace) {
        self.clicked = false;
    }

    fn is_click_held(&self) -> bool {
        self.clicked
    }

    fn debug_string(&self) -> String {
        format!("{:?}", self)
    }

    fn mouse_moved(&mut self, planet: &mut CircleShape, x: i32, _: i32) {
        let hhw = self.handle.dimensions.x / 2.0; //half handle width
        if x as f32 > self.position.x + self.max_width {
            self.handle
                .set_position((self.position.x - hhw + self.max_width, self.position.y));
            self.value = u8::MAX;
        } else if (x as f32) < self.position.x {
            self.handle
                .set_position((self.position.x - hhw, self.position.y));
            self.value = u8::MIN;
        } else {
            self.handle.set_position((x as f32, self.position.y));
            self.value = (self.handle.position().x - self.position.x / self.max_width) as u8;
        }
        let mut color = planet.fill_color();
        match self.color_type {
            ColorType::Red => {
                color.g = self.value;
                color.b = self.value;
            }
            ColorType::Green => {
                color.r = self.value;
                color.b = self.value;
            }
            ColorType::Blue => {
                color.g = self.value;
                color.r = self.value;
            }
            ColorType::Alpha => {
                color.a = self.value;
            }
        }
        planet.set_fill_color(color);
    }
}
