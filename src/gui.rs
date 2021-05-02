use sfml::{
    graphics::{CircleShape, Drawable, Font, RenderTarget, Text, Transformable},
    system::{Vector2, Vector2f},
    SfBox,
};

#[derive(Debug)]
pub struct Gui<'a> {
    pub example_planet: CircleShape<'a>,
    pub held_position: Option<Vector2f>,
    radius: f32,
    size: Vector2<u32>,
    font: &'a SfBox<Font>,
    mass: f32,
    text: Option<Text<'a>>,
}

impl<'a> Gui<'a> {
    pub fn new(size: Vector2<u32>, font: &'a SfBox<Font>) -> Self {
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
            font,
            mass: 30.0,
            text: None,
        }
    }
    pub fn draw(&mut self, target: &mut dyn RenderTarget) {
        if self.held_position.is_none() {
            self.example_planet.draw(target, &Default::default());
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
        self.text
            .as_ref()
            .unwrap()
            .draw(target, &Default::default());
    }
}
