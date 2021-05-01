use sfml::{
    graphics::{CircleShape, Drawable, RenderTarget, Transformable},
    system::Vector2f,
};
pub const DEATH_AGE: usize = 100;
pub struct TrailPoint<'a> {
    x: f32,
    y: f32,
    age: usize,
    circle: CircleShape<'a>,
}

impl TrailPoint<'_> {
    pub fn draw(&self, target: &mut dyn RenderTarget) {
        self.circle.draw(target, &Default::default());
    }
    pub fn new(x: f32, y: f32, other_radius: f32) -> Self {
        let radius = 5.0;
        let mut circle = CircleShape::new(radius, 20);
        circle.set_position(Vector2f::new(
            x - other_radius + radius,
            y - other_radius + radius,
        ));
        TrailPoint {
            x: x - other_radius,
            y: y - other_radius,
            age: 0,
            circle,
        }
    }
    pub fn update(&mut self) -> bool {
        self.age += 1;
        self.age > DEATH_AGE
    }
}
pub fn pyth_thm(a: f32, b: f32) -> f32 {
    (a * a + b * b).sqrt()
}
