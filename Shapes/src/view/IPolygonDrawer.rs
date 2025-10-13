use eframe::egui;
use crate::geometry::polygon::{ConstraintType, Polygon};
use crate::geometry::point::Point;

pub trait IPolygonDrawer {
    fn draw(&self, painter: &egui::Painter, polygon: &mut Polygon);

    fn draw_arc(painter: &egui::Painter, start: Point, end: Point, center: Point, radius: f32, color: egui::Color32) where Self: Sized;
}