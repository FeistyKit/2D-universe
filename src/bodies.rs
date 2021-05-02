use std::{
    error::Error,
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use crate::{trails::TrailPoint, PI, WINDOW_SIZE};
use serde::{Deserialize, Serialize};
use sfml::{
    graphics::{CircleShape, Color, Drawable, RenderStates, RenderTarget, Shape, Transformable},
    system::Vector2f,
};

type Time = f32;
#[derive(Debug)]
pub struct SpaceBody<'a> {
    pub x: f32,
    pub y: f32,
    xv: f32,
    yv: f32,
    ax: f32,
    ay: f32,
    mass: f32,
    radius: f32,
    next_trail: usize,
    shape: CircleShape<'a>,
    immovable: bool,
}
impl SpaceBody<'_> {
    pub fn update_shape_position(&mut self, cam_pos: &Vector2f) {
        self.shape.set_position(Vector2f::new(
            self.x - self.radius - cam_pos.x + WINDOW_SIZE.0 / 2.0,
            self.y - self.radius - cam_pos.y + WINDOW_SIZE.1 / 2.0,
        ));
        let error_margin = 0.1;
        if (self.radius - self.shape.radius()).abs() > error_margin {
            self.shape.set_radius(self.radius);
        }
    }
    pub fn new<'a>(
        position: (f32, f32),
        mass: f32,
        radius: f32,
        xv: f32,
        yv: f32,
        immovable: bool,
        color: Color,
    ) -> SpaceBody<'a> {
        SpaceBody {
            x: position.0,
            y: position.1,
            xv,
            yv,
            ax: 0.0,
            ay: 0.0,
            mass,
            radius,
            next_trail: 10,
            shape: {
                let mut p = CircleShape::new(radius, (radius * PI) as u32);
                p.set_fill_color(color);
                p
            },
            immovable,
        }
    }
    pub fn pos2f(&self) -> Vector2f {
        Vector2f::new(self.x, self.y)
    }
}
#[derive(Debug)]
pub struct WorldSpace<'a> {
    pub bodies: Vec<SpaceBody<'a>>,
    dt: Time,
    gravity: f32,
    softening: f32,
    trails: Vec<TrailPoint<'a>>,
    stopped: bool,
    pub cam_pos: Vector2f,
    pub focused_idx: Option<usize>,
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
            && self.radius == other.radius
    }
}

impl From<SpaceBody<'_>> for BodySerializable {
    fn from(other: SpaceBody<'_>) -> Self {
        let other_color = other.shape.fill_color();
        BodySerializable {
            x: other.x,
            y: other.y,
            xv: other.xv,
            yv: other.yv,
            ax: other.ax,
            ay: other.ay,
            mass: other.mass,
            radius: other.radius,
            immovable: other.immovable,
            color_rgb: (other_color.red(), other_color.green(), other_color.blue()),
        }
    }
}
impl From<BodySerializable> for SpaceBody<'_> {
    fn from(other: BodySerializable) -> Self {
        SpaceBody {
            x: other.x,
            y: other.y,
            xv: other.xv,
            yv: other.yv,
            ax: other.ax,
            ay: other.ay,
            mass: other.mass,
            radius: other.radius,
            immovable: other.immovable,
            next_trail: 10,
            shape: {
                let mut c = CircleShape::new(other.radius, (other.radius * PI) as u32);
                c.set_fill_color(Color::rgb(
                    other.color_rgb.0,
                    other.color_rgb.1,
                    other.color_rgb.2,
                ));
                c
            },
        }
    }
}
#[allow(unused)]
impl<'a> WorldSpace<'a> {
    fn update_positions(&mut self) {
        for planet in self.bodies.iter_mut() {
            if !planet.immovable {
                planet.x += planet.xv * self.dt;
                planet.y += planet.yv * self.dt;
            }
            planet.update_shape_position(&self.cam_pos);
        }
    }
    fn update_time(&mut self) {
        for planet in self.bodies.iter_mut() {
            planet.xv += planet.ax * self.dt;
            planet.yv += planet.ay * self.dt;
        }
    }
    fn update_cam_pos(&mut self) {
        if self.focused_idx.is_some() {
            let body = &self.bodies[self.focused_idx.unwrap()];
            self.cam_pos = body.pos2f();
        } else {
            self.cam_pos = Vector2f::new(0.0, 0.0);
        }
    }
    fn update_trails(&mut self) {
        let mut temp = Vec::new();
        for i in 0..self.trails.len() {
            if self.trails[i].update() {
                temp.push(i);
            }
        }
        for a in temp {
            self.trails.remove(a);
        }
        for planet in &mut self.bodies {
            planet.next_trail -= 1;
            if planet.next_trail < 1 {
                planet.next_trail = 10;
                self.trails.push(TrailPoint::new(planet.x, planet.y));
            }
        }
    }
    fn draw_trails(&mut self, target: &mut dyn RenderTarget) {
        for point in &mut self.trails {
            point.draw(target, self.cam_pos);
        }
    }
    fn update_acceleration(&mut self) {
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
                    let f = (self.gravity * other.mass) / (squared.abs());
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
            trails: Vec::new(),
            stopped: false,
            cam_pos: Vector2f::new(0.0, 0.0),
            focused_idx: None,
        }
    }
    fn draw<'b: 'shader, 'texture, 'shader, 'shader_texture>(
        &'b mut self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        self.draw_trails(target);
        for planet in &self.bodies {
            planet.shape.draw(target, states);
        }
    }
    pub fn serialize<T: AsRef<Path>>(self, p: T) -> Result<(), Box<dyn Error>> {
        let serializable = WorldSpaceSerializable::from(self);
        let serialized = serde_json::to_string(&serializable)?;
        File::create(p)?.write_all(serialized.as_bytes())?;
        Ok(())
    }
    pub fn deserialize<'b, T: AsRef<Path>>(p: T) -> Result<WorldSpace<'b>, Box<dyn Error>> {
        let raw = read_to_string(p)?;
        let space = serde_json::from_str::<WorldSpaceSerializable>(&raw)?;
        Ok(WorldSpace::from(space))
    }
    pub fn stop(&mut self) {
        self.stopped = true;
    }
    pub fn unstop(&mut self) {
        self.stopped = false;
    }
    pub fn switch_stopped(&mut self) {
        self.stopped = !self.stopped;
    }
    pub fn is_stopped(&self) -> bool {
        self.stopped
    }
    pub fn advance(&mut self, target: &mut dyn RenderTarget, states: &RenderStates) {
        if !self.stopped {
            self.update_acceleration();
            self.update_positions();
            self.update_time();
            self.update_trails();
            self.update_cam_pos();
        }
        self.draw(target, states);
        if self.focused_idx.is_some() {
            let body = &self.bodies[self.focused_idx.unwrap()];
        }
    }
    pub fn push_body(&mut self, body: SpaceBody<'a>) {
        self.bodies.push(body);
    }
}
impl Default for WorldSpace<'_> {
    fn default() -> Self {
        let p1 = SpaceBody::new(
            (WINDOW_SIZE.0 / 2.0, WINDOW_SIZE.1 * 3.0 / 4.0),
            50.0,
            30.0,
            -50.0,
            0.0,
            false,
            Color::WHITE,
        );
        let p2 = SpaceBody::new(
            (WINDOW_SIZE.0 / 2.0, WINDOW_SIZE.1 * 5.0 / 8.0),
            50.0,
            30.0,
            50.0,
            0.0,
            false,
            Color::rgb(40, 60, 110),
        );
        WorldSpace::with_bodies(vec![p1, p2])
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct BodySerializable {
    x: f32,
    y: f32,
    xv: f32,
    yv: f32,
    ax: f32,
    ay: f32,
    mass: f32,
    radius: f32,
    immovable: bool,
    color_rgb: (u8, u8, u8),
}
#[derive(Debug, Serialize, Deserialize)]
struct WorldSpaceSerializable {
    dt: Time,
    gravity: f32,
    softening: f32,
    bodies: Vec<BodySerializable>,
    stopped: bool,
    cam_pos: (f32, f32),
    focused_idx: Option<usize>,
}
impl From<WorldSpace<'_>> for WorldSpaceSerializable {
    fn from(other: WorldSpace) -> Self {
        WorldSpaceSerializable {
            dt: other.dt,
            gravity: other.gravity,
            softening: other.softening,
            bodies: other
                .bodies
                .into_iter()
                .map(BodySerializable::from)
                .collect(),
            stopped: other.stopped,
            cam_pos: (other.cam_pos.x, other.cam_pos.y),
            focused_idx: other.focused_idx,
        }
    }
}
impl From<WorldSpaceSerializable> for WorldSpace<'_> {
    fn from(other: WorldSpaceSerializable) -> Self {
        WorldSpace {
            dt: other.dt,
            gravity: other.gravity,
            softening: other.softening,
            bodies: other.bodies.into_iter().map(SpaceBody::from).collect(),
            trails: Vec::new(),
            stopped: other.stopped,
            cam_pos: Vector2f::new(other.cam_pos.0, other.cam_pos.1),
            focused_idx: other.focused_idx,
        }
    }
}
