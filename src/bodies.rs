use std::{
    error::Error,
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use crate::{trails::TrailPoint, PI};
use serde::{Deserialize, Serialize};
use sfml::{
    graphics::{CircleShape, Drawable, RenderStates, RenderTarget, Transformable},
    system::Vector2f,
};

type Time = f32;
#[derive(Debug)]
pub struct SpaceBody<'a> {
    x: f32,
    y: f32,
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
    pub fn update_shape_position(&mut self) {
        self.shape
            .set_position(Vector2f::new(self.x - self.radius, self.y - self.radius));
        let error_margin = 0.1;
        if (self.radius - self.shape.radius()).abs() > error_margin {
            self.shape.set_radius(self.radius);
        }
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
            radius,
            next_trail: 10,
            shape: CircleShape::new(radius, (radius * PI) as u32),
            immovable,
        }
    }
}
#[derive(Debug)]
pub struct WorldSpace<'a> {
    pub bodies: Vec<SpaceBody<'a>>,
    dt: Time,
    gravity: f32,
    softening: f32,
    trails: Vec<TrailPoint<'a>>,
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
            shape: CircleShape::new(other.radius, (other.radius * PI) as u32),
        }
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
    pub fn update_trails(&mut self) {
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
                self.trails
                    .push(TrailPoint::new(planet.x, planet.y, planet.shape.radius()));
            }
        }
    }
    fn draw_trails(&self, target: &mut dyn RenderTarget) {
        for point in &self.trails {
            point.draw(target);
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
        self.draw_trails(target);
    }
    pub fn serialize<T: AsRef<Path>>(self, p: T) -> Result<(), Box<dyn Error>> {
        let serializable = WorldSpaceSerializable::from(self);
        let serialized = serde_json::to_string(&serializable)?;
        File::create(p)?.write_all(serialized.as_bytes())?;
        Ok(())
    }
    pub fn deserialize<'a, T: AsRef<Path>>(p: T) -> Result<WorldSpace<'a>, Box<dyn Error>> {
        let raw = read_to_string(p)?;
        let space = serde_json::from_str::<WorldSpaceSerializable>(&raw)?;
        Ok(WorldSpace::from(space))
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
}
#[derive(Debug, Serialize, Deserialize)]
struct WorldSpaceSerializable {
    dt: Time,
    gravity: f32,
    softening: f32,
    bodies: Vec<BodySerializable>,
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
        }
    }
}
