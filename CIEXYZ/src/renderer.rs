
use eframe::egui::{self, Color32, Align2, FontId};

fn wavelength_to_rgb(wavelength: f32) -> Color32 {
    let gamma = 0.8;
    let intensity_max = 255.0;
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;

    if wavelength >= 380.0 && wavelength < 440.0 {
        r = -(wavelength - 440.0) / (440.0 - 380.0);
        b = 1.0;
    } else if wavelength >= 440.0 && wavelength < 490.0 {
        g = (wavelength - 440.0) / (490.0 - 440.0);
        b = 1.0;
    } else if wavelength >= 490.0 && wavelength < 510.0 {
        g = 1.0;
        b = -(wavelength - 510.0) / (510.0 - 490.0);
    } else if wavelength >= 510.0 && wavelength < 580.0 {
        r = (wavelength - 510.0) / (580.0 - 510.0);
        g = 1.0;
    } else if wavelength >= 580.0 && wavelength < 645.0 {
        r = 1.0;
        g = -(wavelength - 645.0) / (645.0 - 580.0);
    } else if wavelength >= 645.0 && wavelength <= 780.0 {
        r = 1.0;
    }

    let factor = if wavelength >= 380.0 && wavelength < 420.0 {
        0.3 + 0.7 * (wavelength - 380.0) / (420.0 - 380.0)
    } else if wavelength >= 420.0 && wavelength < 645.0 {
        1.0
    } else if wavelength >= 645.0 && wavelength <= 780.0 {
        0.3 + 0.7 * (780.0 - wavelength) / (780.0 - 645.0)
    } else {
        0.0
    };

    let clamp = |v: f32| v.clamp(0.0, 1.0);
    let adjust = |c: f32| if c == 0.0 { 0 } else { (intensity_max * (c * factor).powf(gamma)) as u8 };

    Color32::from_rgb(adjust(clamp(r)), adjust(clamp(g)), adjust(clamp(b)))
}

pub fn draw_chromaticity(
    ui: &mut egui::Ui,
    points: &[(f32, f32)],
    wavelengths: &[f32],
    current_xy: (f32, f32),
    current_rgb: (u8, u8, u8),
) {
    let size = ui.available_size_before_wrap();
    let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
    let rect = response.rect;

    let x_min = 0.0;
    let x_max = 0.8;
    let y_min = 0.0;
    let y_max = 0.9;

    let to_screen = |(x, y): (f32, f32)| {
        let px = rect.min.x + ((x - x_min) / (x_max - x_min)) * rect.width();
        let py = rect.max.y - ((y - y_min) / (y_max - y_min)) * rect.height();
        egui::pos2(px, py)
    };

    // Tło + ramka
    painter.rect_stroke(rect, 0.0, egui::Stroke::new(1.0, Color32::GRAY));

    // Siatka i ticki
    let font = FontId::monospace(11.0);
    let x_ticks = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
    let y_ticks = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    for &tx in &x_ticks {
        let p1 = to_screen((tx, y_min));
        let p2 = to_screen((tx, y_max));
        painter.line_segment([p1, p2], (1.0, Color32::from_gray(50)));
        painter.text(p1, Align2::LEFT_BOTTOM, format!("{:.1}", tx), font.clone(), Color32::LIGHT_GRAY);
    }
    for &ty in &y_ticks {
        let p1 = to_screen((x_min, ty));
        let p2 = to_screen((x_max, ty));
        painter.line_segment([p1, p2], (1.0, Color32::from_gray(50)));
        painter.text(p1, Align2::LEFT_TOP, format!("{:.1}", ty), font.clone(), Color32::LIGHT_GRAY);
    }

    // Trójkąt gamutu sRGB
    let sRGB_points = [(0.64, 0.33), (0.30, 0.60), (0.15, 0.06)];
    for i in 0..3 {
        let p1 = to_screen(sRGB_points[i]);
        let p2 = to_screen(sRGB_points[(i + 1) % 3]);
        painter.line_segment([p1, p2], (2.0, Color32::WHITE));
    }


    // Prostokąt z kolorem wynikowym
    let color_rect = egui::Rect::from_min_size(
        egui::pos2(rect.max.x - 80.0, rect.min.y + 20.0),
        egui::vec2(60.0, 40.0),
    );
    painter.rect_filled(color_rect, 0.0, Color32::from_rgb(current_rgb.0, current_rgb.1, current_rgb.2));
    painter.text(
        color_rect.center(),
        Align2::CENTER_CENTER,
        "sRGB",
        FontId::monospace(14.0),
        Color32::WHITE,
    );

    // Podkowa (kolorowe punkty)
    for ((x, y), wl) in points.iter().zip(wavelengths.iter()) {
        let color = wavelength_to_rgb(*wl);
        painter.circle_filled(to_screen((*x, *y)), 2.0, color);
    }

    // Czarny punkt wynikowy
    painter.circle_filled(to_screen(current_xy), 5.0, Color32::WHITE);
    painter.text(
        to_screen(current_xy),
        Align2::LEFT_TOP,
        format!("x={:.4}, y={:.4}", current_xy.0, current_xy.1),
        font,
        Color32::WHITE,
    );
}




