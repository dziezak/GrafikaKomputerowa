use egui::{Painter, Color32, Pos2, Stroke, Align2};
use crate::geometry::polygon::{Polygon, ConstraintType};

pub fn draw_polygon(painter: &Painter, polygon: &Polygon) {
    // Rysowanie krawędzi
    for (i, window) in polygon.vertices.windows(2).enumerate() {
        let start = &window[0];
        let end = &window[1];

        // Rysowanie odcinka
        painter.line_segment(
            [Pos2::new(start.x, start.y), Pos2::new(end.x, end.y)],
            Stroke::new(2.0, Color32::WHITE),
        );

        // Środek krawędzi
        let mid_x = (start.x + end.x) / 2.0;
        let mid_y = (start.y + end.y) / 2.0;

        // Rysowanie ikony constraintu
        if let Some(constraint) = polygon.constraints.get(i).and_then(|c| *c) {
            let label = match constraint {
                ConstraintType::Horizontal => "H".to_string(),
                ConstraintType::Vertical => "V".to_string(),
                ConstraintType::Diagonal45 => "D".to_string(),
                ConstraintType::FixedLength(len) => format!("{:.1}", len),
            };

            painter.text(
                Pos2::new(mid_x, mid_y),
                Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(14.0),
                Color32::YELLOW,
            );
        }
    }

    // Zamknięcie wielokąta (ostatni z pierwszym)
    if polygon.vertices.len() > 2 {
        let first = &polygon.vertices[0];
        let last = polygon.vertices.last().unwrap();

        painter.line_segment(
            [Pos2::new(last.x, last.y), Pos2::new(first.x, first.y)],
            Stroke::new(2.0, Color32::WHITE),
        );

        // Constraint dla ostatniej krawędzi (łączącej koniec z początkiem)
        if let Some(constraint) = polygon.constraints.last().and_then(|c| *c) {
            let mid_x = (last.x + first.x) / 2.0;
            let mid_y = (last.y + first.y) / 2.0;
            let label = match constraint {
                ConstraintType::Horizontal => "H".to_string(),
                ConstraintType::Vertical => "V".to_string(),
                ConstraintType::Diagonal45 => "D".to_string(),
                ConstraintType::FixedLength(len) => format!("{:.1}", len),
            };
            painter.text(
                Pos2::new(mid_x, mid_y),
                Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(14.0),
                Color32::YELLOW,
            );
        }
    }

    // Rysowanie wierzchołków
    for &v in &polygon.vertices {
        painter.circle_filled(Pos2::new(v.x, v.y), 5.0, Color32::RED);
    }
}
