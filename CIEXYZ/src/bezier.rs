
use eframe::egui::{self, Color32, Align2, FontId, PointerButton};

pub fn bezier_point(control_points: &[(f32, f32)], t: f32) -> (f32, f32) {
    let mut pts = control_points.to_vec();
    let n = pts.len();
    if n == 0 {
        return (0.0, 0.0);
    }
    for r in 1..n {
        for i in 0..(n - r) {
            pts[i].0 = (1.0 - t) * pts[i].0 + t * pts[i + 1].0;
            pts[i].1 = (1.0 - t) * pts[i].1 + t * pts[i + 1].1;
        }
    }
    pts[0]
}


fn sort_by_lambda(points: &mut Vec<(f32, f32)>) {
    points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
}

fn linear_interp(points: &[(f32, f32)], lambda: f32) -> f32 {
    if points.is_empty() {
        return 0.0;
    }
    if points.len() == 1 {
        return points[0].1;
    }

    let mut i = 0usize;
    for j in 0..points.len() - 1 {
        if lambda <= points[j + 1].0 {
            i = j;
            break;
        }
        i = j;
    }
    let (x1, y1) = points[i];
    let (x2, y2) = points[i + 1];

    if (x2 - x1).abs() < f32::EPSILON {
        return y1;
    }

    let u = (lambda - x1) / (x2 - x1);
    y1 + u * (y2 - y1)
}

fn catmull_rom_y(y0: f32, y1: f32, y2: f32, y3: f32, t: f32) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;
    0.5 * ( (2.0 * y1)
        + (-y0 + y2) * t
        + (2.0 * y0 - 5.0 * y1 + 4.0 * y2 - y3) * t2
        + (-y0 + 3.0 * y1 - 3.0 * y2 + y3) * t3 )
}

pub fn evaluate_curve(control_points: &[(f32, f32)], lambda: f32) -> f32 {
    if control_points.is_empty() {
        return 0.0;
    }

    let mut pts = control_points.to_vec();
    sort_by_lambda(&mut pts);

    let x_min = 380.0f32;
    let x_max = 700.0f32;
    let lambda = lambda.clamp(x_min, x_max);

    if pts.len() < 4 {
        return linear_interp(&pts, lambda).clamp(0.0, 1.8);
    }

    let mut i = 0usize;
    for j in 0..pts.len() - 1 {
        if lambda <= pts[j + 1].0 {
            i = j;
            break;
        }
        i = j;
    }

    let p0 = if i == 0 { pts[0] } else { pts[i - 1] };
    let p1 = pts[i];
    let p2 = pts[i + 1];
    let p3 = if i + 2 >= pts.len() { pts[pts.len() - 1] } else { pts[i + 2] };

    let denom = (p2.0 - p1.0);
    let t = if denom.abs() < f32::EPSILON {
        0.0
    } else {
        (lambda - p1.0) / denom
    }.clamp(0.0, 1.0);

    let y = catmull_rom_y(p0.1, p1.1, p2.1, p3.1, t);
    y.clamp(0.0, 1.8)
}


pub fn draw_bezier_interactive(
    ui: &mut egui::Ui,
    control_points: &mut Vec<(f32, f32)>,
    dragging_idx: &mut Option<usize>,
    max_points: usize,
) {
    let size = ui.available_size_before_wrap();
    let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());
    let rect = response.rect;

    // Zakresy osi
    let x_min = 380.0f32;
    let x_max = 700.0f32;
    let y_min = 0.0f32;
    let y_max = 1.8f32;

    // Transformacje
    let to_screen = |(x, y): (f32, f32)| {
        let px = rect.min.x + ((x - x_min) / (x_max - x_min)) * rect.width();
        let py = rect.max.y - ((y - y_min) / (y_max - y_min)) * rect.height();
        egui::pos2(px, py)
    };
    let from_screen = |pos: egui::Pos2| {
        let x = x_min + ((pos.x - rect.min.x) / rect.width()) * (x_max - x_min);
        let y = y_min + ((rect.max.y - pos.y) / rect.height()) * (y_max - y_min);
        (x.clamp(x_min, x_max), y.clamp(y_min, y_max))
    };

    painter.rect_stroke(rect, 0.0, egui::Stroke::new(1.0, Color32::GRAY));

    let font = FontId::monospace(11.0);
    let x_ticks = [380.0, 420.0, 460.0, 500.0, 540.0, 580.0, 620.0, 660.0, 700.0];
    let y_ticks = [0.0, 0.3, 0.6, 0.9, 1.2, 1.5, 1.8];
    for &tx in &x_ticks {
        let p1 = to_screen((tx, y_min));
        let p2 = to_screen((tx, y_max));
        painter.line_segment([p1, p2], (1.0, Color32::from_gray(50)));
        painter.text(p1, Align2::LEFT_BOTTOM, format!("{:.0}", tx), font.clone(), Color32::LIGHT_GRAY);
    }
    for &ty in &y_ticks {
        let p1 = to_screen((x_min, ty));
        let p2 = to_screen((x_max, ty));
        painter.line_segment([p1, p2], (1.0, Color32::from_gray(50)));
        painter.text(p1, Align2::LEFT_TOP, format!("{:.1}", ty), font.clone(), Color32::LIGHT_GRAY);
    }

    if let Some(mouse) = ui.input(|i| i.pointer.hover_pos()) {
        if rect.contains(mouse) {
            painter.line_segment([egui::pos2(mouse.x, rect.min.y), egui::pos2(mouse.x, rect.max.y)],
                                 (1.0, Color32::from_gray(80)));
            painter.line_segment([egui::pos2(rect.min.x, mouse.y), egui::pos2(rect.max.x, mouse.y)],
                                 (1.0, Color32::from_gray(80)));

            let (lx, ly) = from_screen(mouse);
            painter.text(
                egui::pos2(mouse.x + 6.0, mouse.y + 6.0),
                Align2::LEFT_TOP,
                format!("λ = {:.1} nm, I = {:.3}", lx, ly),
                font.clone(),
                Color32::WHITE,
            );
        }
    }

    let pick_radius: f32 = 8.0; // px
    let pointer = ui.input(|i| i.pointer.clone());

    let pointer_pos_opt = pointer.interact_pos();
    let primary_down = pointer.button_down(PointerButton::Primary);
    let primary_clicked = pointer.primary_clicked();
    let primary_released = pointer.button_released(PointerButton::Primary);

    if let Some(pos) = pointer_pos_opt {
        if primary_down && dragging_idx.is_none() {
            let mut nearest: Option<(usize, f32)> = None;
            for (idx, &(x, y)) in control_points.iter().enumerate() {
                let sp = to_screen((x, y));
                let d2 = (sp.x - pos.x).powi(2) + (sp.y - pos.y).powi(2);
                if d2 <= pick_radius.powi(2) {
                    if nearest.map_or(true, |(_, best)| d2 < best) {
                        nearest = Some((idx, d2));
                    }
                }
            }
            if let Some((idx, _)) = nearest {
                *dragging_idx = Some(idx);
            }
        }

        if primary_down {
            if let Some(idx) = *dragging_idx {
                let (nx, ny) = from_screen(pos);
                control_points[idx] = (nx, ny);
            }
        }

        if primary_released {
            *dragging_idx = None;
        }

        if primary_clicked {
            let clicked_on_handle = control_points.iter().any(|&(x, y)| {
                let sp = to_screen((x, y));
                (sp.x - pos.x).abs() <= pick_radius && (sp.y - pos.y).abs() <= pick_radius
            });
            if !clicked_on_handle {
                if control_points.len() < max_points {
                    let (nx, ny) = from_screen(pos);
                    control_points.push((nx, ny));
                } else {
                    ui.painter().text(
                        egui::pos2(rect.min.x + 8.0, rect.min.y + 8.0),
                        Align2::LEFT_TOP,
                        format!("Osiągnięto limit punktów: {}", max_points),
                        FontId::monospace(12.0),
                        Color32::YELLOW,
                    );
                }
            }
        }
    }

    for &(x, y) in control_points.iter() {
        let sp = to_screen((x, y));
        painter.circle_filled(sp, 4.0, Color32::RED);
        painter.circle_stroke(sp, 6.0, egui::Stroke::new(1.0, Color32::WHITE));
    }

    if control_points.len() >= 2 {
        let mut prev = bezier_point(control_points, 0.0);
        for i in 1..=150 {
            let t = i as f32 / 150.0;
            let curr = bezier_point(control_points, t);
            painter.line_segment([to_screen(prev), to_screen(curr)], (2.0, Color32::GREEN));
            prev = curr;
        }
    }
}

pub fn draw_polyline(
    ui: &mut egui::Ui,
    control_points: &mut Vec<(f32, f32)>,
    dragging_idx: &mut Option<usize>,
    max_points: usize,
) {
    let size = ui.available_size_before_wrap();
    let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());
    let rect = response.rect;

    let x_min = 380.0;
    let x_max = 700.0;
    let y_min = 0.0;
    let y_max = 1.8;

    let to_screen = |(x, y): (f32, f32)| {
        let px = rect.min.x + ((x - x_min) / (x_max - x_min)) * rect.width();
        let py = rect.max.y - ((y - y_min) / (y_max - y_min)) * rect.height();
        egui::pos2(px, py)
    };
    let from_screen = |pos: egui::Pos2| {
        let x = x_min + ((pos.x - rect.min.x) / rect.width()) * (x_max - x_min);
        let y = y_min + ((rect.max.y - pos.y) / rect.height()) * (y_max - y_min);
        (x.clamp(x_min, x_max), y.clamp(y_min, y_max))
    };

    // Siatka i osie
    painter.rect_stroke(rect, 0.0, egui::Stroke::new(1.0, Color32::GRAY));
    let font = FontId::monospace(11.0);
    for tx in (380..=700).step_by(40) {
        let p1 = to_screen((tx as f32, y_min));
        let p2 = to_screen((tx as f32, y_max));
        painter.line_segment([p1, p2], (1.0, Color32::from_gray(50)));
        painter.text(p1, Align2::LEFT_BOTTOM, format!("{}", tx), font.clone(), Color32::LIGHT_GRAY);
    }
    for ty in [0.0, 0.3, 0.6, 0.9, 1.2, 1.5, 1.8] {
        let p1 = to_screen((x_min, ty));
        let p2 = to_screen((x_max, ty));
        painter.line_segment([p1, p2], (1.0, Color32::from_gray(50)));
        painter.text(p1, Align2::LEFT_TOP, format!("{:.1}", ty), font.clone(), Color32::LIGHT_GRAY);
    }

    // Obsługa kliknięć
    if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
        let primary_down = ui.input(|i| i.pointer.button_down(PointerButton::Primary));
        let primary_clicked = ui.input(|i| i.pointer.primary_clicked());
        let secondary_clicked = ui.input(|i| i.pointer.button_clicked(PointerButton::Secondary));
        let primary_released = ui.input(|i| i.pointer.button_released(PointerButton::Primary));

        // Drag & drop
        if primary_down && dragging_idx.is_none() {
            for (idx, &(x, y)) in control_points.iter().enumerate() {
                if (to_screen((x, y)).distance(pos)) < 8.0 {
                    *dragging_idx = Some(idx);
                    break;
                }
            }
        }
        if primary_down {
            if let Some(idx) = *dragging_idx {
                control_points[idx] = from_screen(pos);
            }
        }
        if primary_released {
            *dragging_idx = None;
        }

        // Dodawanie punktu lewym kliknięciem
        if primary_clicked && control_points.len() < max_points {
            let clicked_on_handle = control_points.iter().any(|&(x, y)| to_screen((x, y)).distance(pos) < 8.0);
            if !clicked_on_handle {
                control_points.push(from_screen(pos));
            }
        }

        // Usuwanie punktu prawym kliknięciem
        if secondary_clicked {
            if let Some(idx) = control_points.iter().position(|&(x, y)| to_screen((x, y)).distance(pos) < 8.0) {
                control_points.remove(idx);
            }
        }
    }

    // Punkty
    for &(x, y) in control_points.iter() {
        let sp = to_screen((x, y));
        painter.circle_filled(sp, 4.0, Color32::RED);
        painter.circle_stroke(sp, 6.0, egui::Stroke::new(1.0, Color32::WHITE));
    }

    // Krzywa (łączymy punkty linią)
    if control_points.len() >= 2 {
        for i in 0..control_points.len() - 1 {
            painter.line_segment([to_screen(control_points[i]), to_screen(control_points[i + 1])], (2.0, Color32::GREEN));
        }
    }
}

