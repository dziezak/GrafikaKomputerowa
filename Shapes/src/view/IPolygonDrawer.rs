use eframe::egui;
use crate::geometry::polygon::{ConstraintType, Polygon};

pub trait IPolygonDrawer {
    fn draw(&self, painter: &egui::Painter, polygon: &mut Polygon);
}