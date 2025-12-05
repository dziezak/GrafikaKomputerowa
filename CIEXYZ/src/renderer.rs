
use eframe::egui::{self, Align2, Color32, FontId, TextureHandle};
use crate::BgFitMode;

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
    bg_texture: Option<&TextureHandle>,
    bg_mode: BgFitMode,
    bg_opacity: f32,
) {
    // 1) Zarezerwuj dokładnie tyle miejsca, ile panel oferuje
    let size = ui.available_size();
    let (response, painter_root) = ui.allocate_painter(size, egui::Sense::hover());
    let rect = response.rect;

    // 2) Dziedzina osi CIE xy + aspect ratio
    let x_min = 0.0f32;
    let x_max = 0.8f32;
    let y_min = 0.0f32;
    let y_max = 0.9f32;
    let domain_aspect = (x_max - x_min) / (y_max - y_min); // ≈ 0.8889

    // 3) Marginesy wewnątrz panelu (miejsce na etykiety osi / legendę)
    let margin = egui::vec2(48.0, 36.0); // lewy/dolny margines na opisy osi
    let inner = egui::Rect::from_min_max(
        rect.min + egui::vec2(margin.x, margin.y * 0.5),
        rect.max - egui::vec2(margin.x * 0.6, margin.y),
    );

    // 4) Dopasuj wykres do aspect ratio domeny (fit-in)
    let inner_w = inner.width();
    let inner_h = inner.height();
    let inner_aspect = inner_w / inner_h;

    let plot_rect = if inner_aspect > domain_aspect {
        // za szeroki → dopasuj szerokość do wysokości
        let w = inner_h * domain_aspect;
        let x = inner.center().x - w * 0.5;
        egui::Rect::from_min_size(egui::pos2(x, inner.min.y), egui::vec2(w, inner_h))
    } else {
        // za wysoki → dopasuj wysokość do szerokości
        let h = inner_w / domain_aspect;
        let y = inner.center().y - h * 0.5;
        egui::Rect::from_min_size(egui::pos2(inner.min.x, y), egui::vec2(inner_w, h))
    };

    // Painter z klipem do plot_rect
    let painter = painter_root.with_clip_rect(plot_rect);

    // Funkcje mapujące (tylko w obrębie plot_rect)
    let to_screen = |(x, y): (f32, f32)| {
        let px = plot_rect.min.x + ((x - x_min) / (x_max - x_min)) * plot_rect.width();
        let py = plot_rect.max.y - ((y - y_min) / (y_max - y_min)) * plot_rect.height();
        egui::pos2(px, py)
    };

    // === 5) TŁO (obraz) — klipowane do plot_rect
    if let Some(tex) = bg_texture {
        let [tw, th] = tex.size();
        let tw = tw as f32;
        let th = th as f32;
        let tex_aspect = tw / th;
        let rect_aspect = plot_rect.width() / plot_rect.height();

        let mut target = plot_rect;
        let mut uv = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));

        match bg_mode {
            BgFitMode::Stretch => {
                // target = plot_rect, uv = pełne → zniekształca, ale pokrywa całość
            }
            BgFitMode::Contain => {
                // Dopasuj z zachowaniem proporcji (mogą pojawić się „pasy”)
                let scale = (plot_rect.width() / tw).min(plot_rect.height() / th);
                let w = tw * scale;
                let h = th * scale;
                let x = plot_rect.center().x - w * 0.5;
                let y = plot_rect.center().y - h * 0.5;
                target = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(w, h));
            }
            BgFitMode::Cover => {
                // Wypełnij cały wykres, przytnij UV
                if rect_aspect > tex_aspect {
                    // wykres szerszy → przytnij w pionie (V)
                    let new_v = tex_aspect / rect_aspect;
                    let v_margin = (1.0 - new_v) * 0.5;
                    uv.min.y = v_margin;
                    uv.max.y = 1.0 - v_margin;
                } else {
                    // wykres wyższy → przytnij w poziomie (U)
                    let new_u = rect_aspect / tex_aspect;
                    let u_margin = (1.0 - new_u) * 0.5;
                    uv.min.x = u_margin;
                    uv.max.x = 1.0 - u_margin;
                }
            }
        }

        let tint = Color32::from_rgba_unmultiplied(
            255, 255, 255,
            (bg_opacity.clamp(0.0, 1.0) * 255.0) as u8,
        );
        painter.image(tex.id(), target, uv, tint);
    }

    // === 6) Siatka, osie, etykiety — na plot_rect
    painter.rect_stroke(plot_rect, 0.0, egui::Stroke::new(1.0, Color32::GRAY));

    let font = FontId::monospace(11.0);
    let grid_color = Color32::from_gray(70);

    let x_ticks = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
    let y_ticks = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];

    for &tx in &x_ticks {
        let p1 = to_screen((tx, y_min));
        let p2 = to_screen((tx, y_max));
        painter.line_segment([p1, p2], (1.0, grid_color));
        // etykiety osi X pod wykresem (poza klipem, więc użyj painter_root):
        painter_root.text(
            egui::pos2(p1.x, plot_rect.max.y + 14.0),
            Align2::CENTER_CENTER,
            format!("{:.1}", tx),
            font.clone(),
            Color32::LIGHT_GRAY,
        );
    }
    for &ty in &y_ticks {
        let p1 = to_screen((x_min, ty));
        let p2 = to_screen((x_max, ty));
        painter.line_segment([p1, p2], (1.0, grid_color));
        // etykiety osi Y z lewej (poza klipem — painter_root):
        painter_root.text(
            egui::pos2(plot_rect.min.x - 18.0, p1.y),
            Align2::RIGHT_CENTER,
            format!("{:.1}", ty),
            font.clone(),
            Color32::LIGHT_GRAY,
        );
    }

    // Opisy osi
    painter_root.text(
        egui::pos2(plot_rect.center().x, plot_rect.max.y + 32.0),
        Align2::CENTER_CENTER,
        "x – chromaticity",
        FontId::proportional(12.0),
        Color32::WHITE,
    );
    painter_root.text(
        egui::pos2(plot_rect.min.x - 36.0, plot_rect.center().y),
        Align2::RIGHT_CENTER,
        "y – chromaticity",
        FontId::proportional(12.0),
        Color32::WHITE,
    );

    // === 7) Brzeg podkowy (kolorowy)
    for ((x, y), wl) in points.iter().zip(wavelengths.iter()) {
        let color = wavelength_to_rgb(*wl);
        painter.circle_filled(to_screen((*x, *y)), 2.0, color);
    }

    // === 8) Trójkąt gamutu sRGB
    let srgb = [(0.64, 0.33), (0.30, 0.60), (0.15, 0.06)];
    for i in 0..3 {
        let p1 = to_screen(srgb[i]);
        let p2 = to_screen(srgb[(i + 1) % 3]);
        painter.line_segment([p1, p2], (2.0, Color32::WHITE));
    }

    // === 9) Punkt wynikowy + próbka koloru
    let xy_screen = to_screen(current_xy);
    painter.circle_filled(xy_screen, 5.5, Color32::BLACK);
    painter.circle_stroke(xy_screen, 7.5, egui::Stroke::new(1.0, Color32::WHITE));

    // próbka koloru (poza klipem — painter_root)
    let sample_rect = egui::Rect::from_min_size(
        egui::pos2(plot_rect.max.x - 86.0, plot_rect.min.y + 10.0),
        egui::vec2(70.0, 40.0),
    );
    painter_root.rect_filled(
        sample_rect,
        4.0,
        Color32::from_rgb(current_rgb.0, current_rgb.1, current_rgb.2),
    );
    painter_root.rect_stroke(sample_rect, 4.0, egui::Stroke::new(1.0, Color32::WHITE));
    painter_root.text(
        sample_rect.center() + egui::vec2(0.0, 28.0),
        Align2::CENTER_CENTER,
        "sRGB",
        FontId::monospace(12.0),
        Color32::WHITE,
    );

    // 10) Zewnętrzna ramka (obszar panelu) — opcjonalnie
    painter_root.rect_stroke(rect, 0.0, egui::Stroke::new(1.0, Color32::from_gray(150)));
}





