use sfml::{graphics::RenderTarget, system::Vector2f};
use std::{fmt::Debug, usize};

use crate::{bodies::WorldSpace, gui::Gui};
#[derive(Debug)]
pub enum WidgetKind {
    TestButton(String),
}

pub trait Widget {
    fn get_bounds(&self) -> (Vector2f, Vector2f);
    fn get_layer(&self) -> usize;
    fn draw(&self, target: &mut dyn RenderTarget);
    fn widget_type(&self) -> WidgetKind;
    fn click(&mut self, gui: &Gui, space: &mut WorldSpace);
    fn release_click(&mut self, gui: &Gui, space: &mut WorldSpace);
    fn has_been_clicked(&self) -> bool;
}
impl PartialOrd for dyn Widget {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_layer().partial_cmp(&other.get_layer())
    }
}
impl PartialEq for dyn Widget {
    fn eq(&self, other: &Self) -> bool {
        self.get_layer() == other.get_layer()
    }
}
impl Debug for dyn Widget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.widget_type() {
            WidgetKind::TestButton(s) => write!(f, "{}", s),
        }
    }
}
impl Ord for dyn Widget {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_layer().cmp(&other.get_layer())
    }
}
impl Eq for dyn Widget {}
