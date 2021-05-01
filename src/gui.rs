use sfml::{
    graphics::{CircleShape, Drawable, RenderTarget, Transformable},
    system::{Vector2, Vector2f},
};
#[derive(Debug)]
pub struct Gui<'a> {
    pub example_planet: CircleShape<'a>,
    pub held_position: Option<Vector2f>,
    radius: f32,
    size: Vector2<u32>,
}

impl Gui<'_> {
    pub fn new(size: Vector2<u32>) -> Self {
        let mut circle = CircleShape::new(30.0, 100);
        let default_radius = 30;
        circle.set_position((
            (default_radius) as f32,
            (size.y - 2 * default_radius) as f32,
        ));
        Gui {
            example_planet: circle,
            held_position: None,
            radius: 30.0,
            size,
        }
    }
    pub fn draw(&self, target: &mut dyn RenderTarget) {
        if self.held_position.is_none() {
            self.example_planet.draw(target, &Default::default());
        }
    }
}
