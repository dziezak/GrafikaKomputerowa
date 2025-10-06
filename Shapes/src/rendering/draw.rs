use egui::{Painter, Color32, Pos2, Stroke};
use crate::geometry::polygon::Polygon;

pub fn draw_polygon(painter: &Painter, polygon: &Polygon) {
    // Rysujemy krawędzie wielokąta
    for window in polygon.vertices.windows(2) {
        let start = Pos2::new(window[0].x as f32, window[0].y as f32);
        let end = Pos2::new(window[1].x as f32, window[1].y as f32);
        painter.line_segment([start, end], Stroke::new(2.0, Color32::WHITE));
    }

    // Zamknięcie wielokąta
    if polygon.vertices.len() > 2 {
        let first = Pos2::new(polygon.vertices[0].x as f32, polygon.vertices[0].y as f32);
        let last = Pos2::new(
            polygon.vertices[polygon.vertices.len() - 1].x as f32,
            polygon.vertices[polygon.vertices.len() - 1].y as f32,
        );
        painter.line_segment([first, last], Stroke::new(2.0, Color32::WHITE));
    }

    // Rysowanie wierzchołków
    for v in &polygon.vertices {
        let pos = Pos2::new(v.x as f32, v.y as f32);
        painter.circle_filled(pos, 5.0, Color32::RED);
    }
}
