use eframe::egui;
use crate::geometry::polygon::{ConstraintType, Polygon};
use crate::geometry::point::Point;

pub trait IPolygonDrawer {
    fn draw(&self, painter: &egui::Painter, polygon: &mut Polygon);
    fn draw_arc(
        painter: &egui::Painter,
        center: Point,
        radius: f32,
        start: Point,
        end: Point,
        clockwise: bool,
    ) where Self: Sized;
}