use std::{
    collections::{BTreeSet, VecDeque},
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
const ERROR_MARGIN: f32 = 0.01;
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
    pub shape: CircleShape<'a>,
    immovable: bool,
    index: usize,
}
impl Eq for SpaceBody<'_> {}
impl PartialEq for SpaceBody<'_> {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() > ERROR_MARGIN
            && (self.y - other.y).abs() > ERROR_MARGIN
            && (self.xv - other.xv).abs() > ERROR_MARGIN
            && (self.mass - other.mass).abs() > ERROR_MARGIN
            && (self.radius - other.radius).abs() > ERROR_MARGIN
            && (self.immovable == other.immovable)
    }
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
    #[allow(clippy::clippy::too_many_arguments)]
    pub fn new<'a>(
        position: (f32, f32),
        mass: f32,
        radius: f32,
        xv: f32,
        yv: f32,
        immovable: bool,
        color: Color,
        index: usize,
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
            index,
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
    trails: VecDeque<TrailPoint<'a>>,
    stopped: bool,
    pub cam_pos: Vector2f,
    pub focused_idx: Option<usize>,
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
            color_rgb: (other_color.r, other_color.g, other_color.b),
            index: other.index,
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
            index: other.index,
        }
    }
}
impl<'a> WorldSpace<'a> {
    pub fn validate(&mut self) {
        if !self.bodies.is_empty() {
            for i in 0..self.bodies.len() {
                self.bodies[i].index = i;
            }
        }
    }
    fn update_positions(&mut self) {
        for planet in self.bodies.iter_mut() {
            if !planet.immovable {
                planet.x += planet.xv * self.dt;
                planet.y += planet.yv * self.dt;
            }
        }
    }
    fn update_planets_shape_pos(&mut self) {
        for planet in &mut self.bodies {
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
        if let Some(idx) = self.focused_idx {
            if let Some(real) = self.get_nearest_index(idx) {
                let body = &self.bodies[real];
                self.cam_pos = body.pos2f();
            }
        }
    }
    fn get_nearest_index(&self, index: usize) -> Option<usize> {
        if self.bodies.is_empty() {
            return None;
        }
        if self.bodies.get(index).is_some() {
            Some(index)
        } else {
            self.get_nearest_index(index - 1)
        }
    }
    fn collide(&self, idx_a: usize, idx_b: usize) -> SpaceBody<'a> {
        assert!(self.bodies.get(idx_a).is_some());
        assert!(self.bodies.get(idx_b).is_some());
        assert_ne!(idx_a, idx_b);
        let body_a = self.bodies.get(idx_a).unwrap();
        let body_b = &self.bodies[idx_b];
        let total_mass = body_a.mass + body_b.mass;
        let (r, g, b) = (
            ((body_a.shape.fill_color().r as f32 * body_a.mass
                + body_b.shape.fill_color().r as f32 * body_b.mass)
                / total_mass) as u8,
            ((body_a.shape.fill_color().g as f32 * body_a.mass
                + body_b.shape.fill_color().g as f32 * body_b.mass)
                / total_mass) as u8,
            ((body_a.shape.fill_color().b as f32 * body_a.mass
                + body_b.shape.fill_color().b as f32 * body_b.mass)
                / total_mass) as u8,
        );
        let radius = (body_a.radius * body_a.radius + body_b.radius * body_b.radius).sqrt();
        let xv = (body_a.xv * body_a.mass + body_b.xv * body_b.mass) / total_mass;
        let yv = (body_a.yv * body_a.mass + body_b.yv * body_b.mass) / total_mass;
        let position = (
            body_a.x / 2.0 + body_b.x / 2.0,
            body_a.y / 2.0 + body_b.y / 2.0,
        );
        let p = SpaceBody::new(
            position,
            total_mass,
            radius,
            xv,
            yv,
            false,
            Color::rgb(r, g, b),
            self.bodies.len(),
        );
        println!("{:?}", p);
        p
    }
    pub fn check_for_collisions(&mut self) {
        if self.bodies.is_empty() || self.bodies.len() == 1 {
            return;
        }
        let mut to_remove = BTreeSet::new();
        let mut to_push = Vec::new();
        let t = self.bodies.len();
        let mut new_focused = None;
        for a in 0..t {
            for b in 0..t {
                if a != b
                    && (self.bodies[a].radius + self.bodies[b].radius).powi(2)
                        > (self.bodies[a].x - self.bodies[b].x).powi(2)
                            + (self.bodies[a].y - self.bodies[b].y).powi(2)
                    && !to_remove.contains(&a)
                    && !to_remove.contains(&b)
                {
                    to_remove.insert(a);
                    to_remove.insert(b);
                    let p = self.collide(a, b);
                    if !to_push.contains(&p) {
                        if let Some(idx) = self.focused_idx {
                            if a == idx {
                                new_focused = Some(to_push.len());
                            }
                            if b == idx {
                                new_focused = Some(to_push.len());
                            }
                        }
                        to_push.push(p);
                    } else if let Some(idx) = self.focused_idx {
                        if a == idx {
                            new_focused = Some(to_push.iter().position(|x| x == &p).unwrap());
                        }
                        if b == idx {
                            new_focused = Some(to_push.iter().position(|x| x == &p).unwrap());
                        }
                    }
                }
            }
        }
        for p in to_remove.iter().enumerate() {
            self.bodies.remove(p.1 - p.0);
        }
        #[allow(clippy::needless_range_loop)]
        for a in 0..to_push.len() {
            to_push[a].index = a;
        }
        let mut q = self.bodies.len();
        for mut i in to_push {
            if let Some(idx) = new_focused {
                println!("{:?}, {}", new_focused, i.index);
                if idx == i.index {
                    self.focused_idx = Some(self.bodies.len())
                }
            }
            i.index = q;
            self.bodies.push(i);
            q += 1;
        }
    }
    pub fn clear_bodies(&mut self) {
        self.bodies = Vec::new();
        self.focused_idx = None;
    }
    fn update_trails(&mut self) {
        let mut temp = 0;
        for i in 0..self.trails.len() {
            if self.trails[i].update() {
                temp += 1;
            }
        }
        for _ in 0..temp {
            self.trails.pop_front();
        }
        for planet in &mut self.bodies {
            planet.next_trail -= 1;
            if planet.next_trail < 1 {
                planet.next_trail = 10;
                self.trails.push_back(TrailPoint::new(planet.x, planet.y));
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
                let error_margin = 1.0;
                if (other.x - planet.x).abs() > error_margin
                    && (other.y - planet.y).abs() > error_margin
                {
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
            trails: VecDeque::new(),
            stopped: false,
            cam_pos: Vector2f::new(WINDOW_SIZE.0 / 2.0, WINDOW_SIZE.1 * 0.5),
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
            planet.shape.draw(target, *states);
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
    pub fn switch_stopped(&mut self) {
        self.stopped = !self.stopped;
    }
    pub fn advance(&mut self, target: &mut dyn RenderTarget, states: &RenderStates) {
        if !self.stopped {
            self.check_for_collisions();
            self.update_acceleration();
            self.update_positions();
            self.update_time();
            self.update_trails();
        }
        self.update_cam_pos();
        self.update_planets_shape_pos();
        self.draw(target, states);
    }
    pub fn push_body(&mut self, body: SpaceBody<'a>) {
        self.bodies.push(body);
    }
    pub fn prepare_for_gui(&mut self) -> Option<(CircleShape<'a>, usize)> {
        if let Some(index) = self.focused_idx {
            if let Some(real) = self.get_nearest_index(index) {
                self.focused_idx = Some(real);
                Some((self.bodies[real].shape.clone(), real))
            } else {
                self.focused_idx = None;
                None
            }
        } else {
            None
        }
    }
    pub fn advance_focused_idx(&mut self) {
        if self.bodies.is_empty() {
            return;
        }
        let max = self.bodies.len() - 1;
        if let Some(index) = self.focused_idx {
            if index < max {
                self.focused_idx = Some(index + 1);
            } else {
                self.focused_idx = None;
            }
        } else {
            self.focused_idx = Some(0);
        }
    }
    pub fn reduce_focused_index(&mut self) {
        if self.bodies.is_empty() {
            return;
        }
        let max = self.bodies.len() - 1;
        if let Some(index) = self.focused_idx {
            if index > 0 {
                self.focused_idx = Some(index - 1);
            } else {
                self.focused_idx = None;
            }
        } else {
            self.focused_idx = Some(max);
        }
    }
    pub fn remove_selected(&mut self) {
        if let Some(index) = self.focused_idx {
            self.remove_body(index);
            if self.bodies.is_empty() {
                self.focused_idx = None;
            }
        }
    }
    pub fn remove_body(&mut self, idx: usize) {
        self.bodies.remove(idx);
        for planet in &mut self.bodies[idx..] {
            planet.index -= 1;
        }
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
            0,
        );
        let p2 = SpaceBody::new(
            (WINDOW_SIZE.0 / 2.0, WINDOW_SIZE.1 * 5.0 / 8.0),
            50.0,
            30.0,
            50.0,
            0.0,
            false,
            Color::rgb(40, 60, 110),
            1,
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
    index: usize,
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
            trails: VecDeque::new(),
            stopped: other.stopped,
            cam_pos: Vector2f::new(other.cam_pos.0, other.cam_pos.1),
            focused_idx: other.focused_idx,
        }
    }
}
