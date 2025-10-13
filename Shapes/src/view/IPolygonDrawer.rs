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

    fn draw_arc(
        &self,
        painter: &egui::Painter,
        start: crate::geometry::point::Point,
        end: crate::geometry::point::Point,
        center: crate::geometry::point::Point,
        radius: f32,
        stroke: egui::Stroke,
        clockwise: bool,
    );

}