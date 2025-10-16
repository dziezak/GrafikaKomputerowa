use std::f32;
use eframe::egui;
use eframe::emath::Pos2;
use eframe::epaint::Color32;
use egui::{Painter, Stroke};
use crate::geometry::polygon::{ConstraintType, Polygon};
use crate::geometry::point::Point;

pub trait IPolygonDrawer {
    fn draw(&self, painter: &egui::Painter, polygon: &mut Polygon);


    fn draw_arc_between_points(
        &self,
        painter: &Painter,
        p1: Pos2,
        p2: Pos2,
        arc_angle: f32, // np. std::f32::consts::PI dla półokręgu
        color: Color32,
        thickness: f32,
    );


    fn compute_arc_geometry(
        start: Point,
        end: Point,
        tangent_start: Option<Point>, // punkt kierunku dla G1 start
        tangent_end: Option<Point>,   // punkt kierunku dla G1 end
        g1_start: bool,
        g1_end: bool,
    ) -> (Point, f32) where Self: Sized;
    fn draw_cubic_bezier(
        &self,
        painter: &egui::Painter,
        p0: Point,
        p1: Point,
        p2: Point,
        p3: Point,
        stroke: egui::Stroke,
    );
    fn draw_dashed_polyline(&self, painter: &egui::Painter, pts: &[egui::Pos2], stroke: egui::Stroke);
}