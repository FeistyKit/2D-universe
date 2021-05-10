use sfml::{
    graphics::{CircleShape, Color, RectangleShape, Shape, Transformable},
    system::Vector2f,
};
use std::f32::consts::PI;
#[derive(Debug)]
pub struct RoundedRect<'a> {
    pub radius: f32,
    pub circles: [CircleShape<'a>; 4], //circles are in order like quadrants of coordinate grid
    pub rectangles: [RectangleShape<'a>; 2],
    pub dimensions: Vector2f,
    pub position: Vector2f,
    pub color: Color,
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
    fn update_positions(&mut self) {
        let position = self.position;
        let dimensions = self.dimensions;
        let radius = self.radius;
        self.circles[0].set_position((position.x + dimensions.x - radius * 2.0, position.y));
        self.circles[1].set_position(position);
        self.circles[2].set_position((position.x, position.y + dimensions.y - 2.0 * radius));
        self.circles[3].set_position((
            position.x + dimensions.x - radius * 2.0,
            position.y + dimensions.y - radius * 2.0,
        ));
        self.rectangles[0].set_position((position.x + radius, position.y));
        self.rectangles[1].set_position((position.x, position.y + radius));
    }
    #[allow(unused)]
    pub fn set_fill_color(&mut self, color: Color) {
        self.color = color;
        self.update_color();
    }
    #[allow(unused)]
    fn update_color(&mut self) {
        for circle in &mut self.circles {
            circle.set_fill_color(self.color);
        }
        for rect in &mut self.rectangles {
            rect.set_fill_color(self.color);
        }
    }
    #[allow(unused)]
    fn change_radius(&mut self, radius: f32) {
        *self = RoundedRect::new(radius, self.dimensions, self.position, self.color);
    }
}
impl Transformable for RoundedRect<'_> {
    fn set_position<P: Into<Vector2f>>(&mut self, bposition: P) {
        self.position = bposition.into();
        self.update_positions();
    }

    fn set_rotation(&mut self, _angle: f32) {
        unimplemented!()
    }

    fn set_scale<S: Into<Vector2f>>(&mut self, _scale: S) {
        unimplemented!()
    }

    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        self.position = origin.into() - self.dimensions / 2.0;
        self.update_positions();
    }

    fn position(&self) -> Vector2f {
        self.position
    }

    fn rotation(&self) -> f32 {
        unimplemented!()
    }

    fn get_scale(&self) -> Vector2f {
        unimplemented!()
    }

    fn origin(&self) -> Vector2f {
        self.position + 2.0 * self.dimensions
    }

    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        self.position += offset.into();
    }

    fn rotate(&mut self, _angle: f32) {
        unimplemented!()
    }

    fn scale<F: Into<Vector2f>>(&mut self, _factors: F) {
        unimplemented!()
    }

    fn transform(&self) -> sfml::graphics::Transform {
        unimplemented!()
    }

    fn inverse_transform(&self) -> sfml::graphics::Transform {
        unimplemented!()
    }
}
