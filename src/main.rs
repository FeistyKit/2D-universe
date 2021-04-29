use sfml::{
    graphics::{
        CircleShape, Color, Drawable, RenderStates, RenderTarget, RenderWindow, Transformable,
    },
    system::Vector2f,
    window::{Event, Style},
};
use std::f32::consts::PI;
fn main() {
    let p1 = SpaceBody::new(90.0, 90.0, 1000.0, 50.0, 20.0, 10.0, false);
    let p2 = SpaceBody::new(500.0, 500.0, 500.0, 25.0, -10.0, 10.0, true);
    let mut space = WorldSpace::with_bodies(vec![p1, p2]);
    let mut window = RenderWindow::new(
        (1600, 1600),
        "Universe simulator",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(45);
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }
        window.set_active(true);
        window.clear(Color::BLACK);
        space.update_acceleration();
        space.update_positions();
        space.update_time();
        space.draw(&mut window, &Default::default());
        window.display();
    }
}
type Time = f32;
#[derive(Debug)]
struct SpaceBody<'a> {
    x: f32,
    y: f32,
    xv: f32,
    yv: f32,
    ax: f32,
    ay: f32,
    mass: f32,
    shape: CircleShape<'a>,
    immovable: bool,
}
impl SpaceBody<'_> {
    pub fn update_shape_position(&mut self) {
        self.shape
            .set_position(Vector2f::new(self.x as f32, self.y as f32));
    }
    pub fn new<'a>(
        x: f32,
        y: f32,
        mass: f32,
        radius: f32,
        xv: f32,
        yv: f32,
        immovable: bool,
    ) -> SpaceBody<'a> {
        SpaceBody {
            x,
            y,
            xv,
            yv,
            ax: 0.0,
            ay: 0.0,
            mass,
            shape: CircleShape::new(radius, (radius * PI) as u32),
            immovable,
        }
    }
}
struct WorldSpace<'a> {
    bodies: Vec<SpaceBody<'a>>,
    dt: Time,
    gravity: f32,
    softening: f32,
}

impl PartialEq for SpaceBody<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.xv == other.xv
            && self.yv == other.yv
            && self.ax == other.ax
            && self.ay == other.ay
            && self.mass == other.mass
    }
}

impl WorldSpace<'_> {
    pub fn update_positions(&mut self) {
        for planet in self.bodies.iter_mut() {
            if !planet.immovable {
                planet.x += planet.xv * self.dt;
                planet.y += planet.yv * self.dt;
            }
            planet.update_shape_position();
        }
    }
    pub fn update_time(&mut self) {
        for planet in self.bodies.iter_mut() {
            planet.xv += planet.ax * self.dt;
            planet.yv += planet.ay * self.dt;
        }
    }
    pub fn update_acceleration(&mut self) {
        let len = self.bodies.len();
        for i in 0..len {
            let mut ax = 0.0;
            let mut ay = 0.0;
            let planet = &self.bodies[i];
            for b in 0..len {
                let other = &self.bodies[b];
                if other != planet {
                    let dx = other.x - planet.x;
                    let dy = other.y - planet.y;
                    let squared = dx * dx + dy * dy;
                    let f =
                        (self.gravity * other.mass) / (squared * (squared + self.softening).sqrt());
                    ax += dx * f;
                    ay += dy * f;
                }
            }
            let mut planet_mut = self.bodies.get_mut(i).unwrap();
            planet_mut.ax = ax;
            planet_mut.ay = ay;
        }
    }
    pub fn with_bodies(bodies: Vec<SpaceBody>) -> WorldSpace {
        WorldSpace {
            bodies,
            gravity: 70.0,
            dt: 0.1,
            softening: 0.15,
        }
    }
    pub fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        for planet in &self.bodies {
            planet.shape.draw(target, states);
        }
    }
}
