use sfml::{
    graphics::{CircleShape, Drawable, RenderTarget, Transformable},
    system::Vector2f,
};

use crate::WINDOW_SIZE;
pub const DEATH_AGE: usize = 100;
#[derive(Debug)]
pub struct TrailPoint<'a> {
    age: usize,
    circle: CircleShape<'a>,
    x: f32,
    y: f32,
}

impl TrailPoint<'_> {
    pub fn draw(&mut self, target: &mut dyn RenderTarget, cam_pos: Vector2f) {
        self.circle.set_position((
            self.x - self.circle.radius() - cam_pos.x + WINDOW_SIZE.0 / 2.0,
            self.y - self.circle.radius() - cam_pos.y + WINDOW_SIZE.1 / 2.0,
        ));
        self.circle.draw(target, &Default::default());
    }
    pub fn new(x: f32, y: f32) -> Self {
        let radius = 5.0;
        let mut circle = CircleShape::new(radius, 20);
        circle.set_position(Vector2f::new(x, y));
        TrailPoint {
            age: 0,
            circle,
            x,
            y,
        }
    }
    pub fn update(&mut self) -> bool {
        self.age += 1;
        self.age > DEATH_AGE
    }
}
#[allow(unused)]
pub fn pyth_thm(a: f32, b: f32) -> f32 {
    (a * a + b * b).sqrt()
}
