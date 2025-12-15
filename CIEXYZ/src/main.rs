mod data_loader;
mod cie;
mod renderer;
mod bezier;
mod serialization;

use eframe::egui::{self, TextureFilter};
use egui::TextureHandle;
use data_loader::load_xyz_data;
use cie::xyz_to_xy;
use serde::{Serialize, Deserialize};
use crate::serialization::MyAppState;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CIE Chromaticity Diagram",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum BgFitMode {
    Stretch,
    Contain,
    Cover,
}
impl Default for BgFitMode {
    fn default() -> Self {
        BgFitMode::Cover
    }
}

struct MyApp {
    // Lewy panel: podkowa
    points: Vec<(f32, f32)>,
    wavelengths: Vec<f32>,
    current_xy: (f32, f32),
    current_rgb: (u8, u8, u8),

    // Prawy panel: krzywa Béziera (rozklad widmowy)
    control_points: Vec<(f32, f32)>,
    dragging_idx: Option<usize>,
    max_points: usize,

    // dane do wykresu ( gdzies trzeba o wczytac cn?)
    xyz_samples: Vec<(f32, f32, f32, f32)>,

    // Tlo pod wykresem chromatycznosci
    bg_texture: Option<TextureHandle>,
    bg_mode: BgFitMode,
    bg_opacity: f32,

    //Laboaltoria
    curve: bool,
}


impl Default for MyApp {
    fn default() -> Self {
        let xyz_samples = load_xyz_data("src/assets/data.txt").unwrap();

        let mut points = Vec::new();
        let mut wavelengths = Vec::new();

        for (wl, x, y, z) in &xyz_samples {
            points.push(xyz_to_xy(*x, *y, *z));
            wavelengths.push(*wl);
        }

        Self {
            points,
            wavelengths,
            current_xy: (0.33, 0.33),
            current_rgb: (0, 0, 0),
            control_points: vec![(400.0, 0.6), (500.0, 1.2), (620.0, 0.8)],
            dragging_idx: None,
            max_points: 6,
            xyz_samples,
            bg_texture: None,
            bg_mode: BgFitMode::Contain,
            bg_opacity: 0.35,
            curve: true,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (xy, rgb) = self.compute_color_from_curve();
        self.current_xy = xy;
        self.current_rgb = rgb;

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .min_width(280.0)
            .max_width(800.0)
            .default_width(520.0)
            .show(ctx, |ui| {
                ui.heading("CIE 1931 Chromaticity Diagram");
                ui.horizontal(|ui| {
                    if ui.button("Wczytaj tło…").clicked() {
                        self.pick_and_load_bg(ctx);
                    }
                    ui.label("Tryb dopasowania:");
                    ui.radio_value(&mut self.bg_mode, BgFitMode::Contain, "Contain");
                    ui.radio_value(&mut self.bg_mode, BgFitMode::Cover, "Cover");
                    ui.radio_value(&mut self.bg_mode, BgFitMode::Stretch, "Stretch");
                });
                ui.add(egui::Slider::new(&mut self.bg_opacity, 0.0..=1.0).text("Przezroczystość tła"));
                ui.separator();

                renderer::draw_chromaticity(
                    ui,
                    &self.points,
                    &self.wavelengths,
                    self.current_xy,
                    self.current_rgb,
                    self.bg_texture.as_ref(),
                    self.bg_mode,
                    self.bg_opacity,
                );

                ui.separator();
                ui.label(format!(
                    "xy: ({:.4}, {:.4}) | sRGB: ({}, {}, {})",
                    xy.0, xy.1, rgb.0, rgb.1, rgb.2
                ));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rozkład widmowy (Catmull–Rom spline)");
            ui.horizontal(|ui| {
                ui.label("Limit punktów:");
                ui.add(egui::DragValue::new(&mut self.max_points).clamp_range(1..=16));
                if ui.button("Resetuj punkty").clicked() {
                    self.control_points = vec![(400.0, 0.6), (500.0, 1.2), (620.0, 0.8)];
                    self.dragging_idx = None;
                }
                if ui.button("krzywa").clicked() {
                    self.curve = true;
                }
                if ui.button("łamana").clicked() {
                    self.curve = false;
                }
            ui.horizontal(|ui| {
                    if ui.button("Zapisz stan…").clicked() {
                        let _ = self.save_state("app_state.json");
                    }
                    if ui.button("Wczytaj stan…").clicked() {
                        let _ = self.load_state("app_state.json");
                    }
                });

        });
            ui.separator();

            if self.curve {
                bezier::draw_bezier_interactive(
                    ui,
                    &mut self.control_points,
                    &mut self.dragging_idx,
                    self.max_points,
                );
            } else {
                bezier::draw_polyline(
                    ui,
                    &mut self.control_points,
                    &mut self.dragging_idx,
                    self.max_points,
                );
            }
        });
    }
}

impl MyApp {
    pub fn compute_color_from_curve(&self) -> ((f32, f32), (u8, u8, u8)) {
        let mut X = 0.0f32;
        let mut Y = 0.0f32;
        let mut Z = 0.0f32;
        let mut total_intensity = 0.0f32;

        // sample co 5 nm od 380 do 700
        for wl in (380..=700).step_by(5) {
            let wl_f = wl as f32;
            let intensity = bezier::evaluate_curve(&self.control_points, wl_f);
            if intensity <= 0.0 {
                continue;
            }

            let (x_bar, y_bar, z_bar) = self.xyz_at_wavelength(wl_f);
            X += intensity * x_bar;
            Y += intensity * y_bar;
            Z += intensity * z_bar;
            total_intensity += intensity;
        }

        if total_intensity == 0.0 {
            return ((0.0, 0.0), (0, 0, 0));
        }

        X /= total_intensity;
        Y /= total_intensity;
        Z /= total_intensity;

        let sum = X + Y + Z;
        if sum <= 0.0 {
            return ((0.0, 0.0), (0, 0, 0));
        }
        let x = X / sum;
        let y = Y / sum;

        let (x_clamped, y_clamped) = self.clamp_xy_to_srgb_gamut(x, y);

        let X2;
        let Z2;
        if y_clamped > 0.0 {
            X2 = (x_clamped * Y) / y_clamped;
            Z2 = ((1.0 - x_clamped - y_clamped) * Y) / y_clamped;
        } else {
            return ((x_clamped, y_clamped), (0, 0, 0));
        }

        let rgb = cie::xyz_to_srgb(X2, Y, Z2);
        ((x_clamped, y_clamped), rgb)
    }

    pub fn xyz_at_wavelength(&self, wl: f32) -> (f32, f32, f32) {
        if self.xyz_samples.len() < 2 {
            return (0.0, 0.0, 0.0);
        }

        for i in 0..self.xyz_samples.len().saturating_sub(1) {
            let (w1, x1, y1, z1) = self.xyz_samples[i];
            let (w2, x2, y2, z2) = self.xyz_samples[i + 1];

            if wl >= w1 && wl <= w2 {
                let denom = w2 - w1;
                if denom.abs() < f32::EPSILON {
                    return (x1, y1, z1);
                }
                let t = (wl - w1) / denom;
                return (
                    x1 + t * (x2 - x1),
                    y1 + t * (y2 - y1),
                    z1 + t * (z2 - z1),
                );
            }
        }

        let first = self.xyz_samples.first().unwrap();
        let last = self.xyz_samples.last().unwrap();
        if wl < first.0 {
            (first.1, first.2, first.3)
        } else {
            (last.1, last.2, last.3)
        }
    }


    pub fn point_in_triangle(
        &self,
        p: (f32, f32),
        a: (f32, f32),
        b: (f32, f32),
        c: (f32, f32),
    ) -> bool {
        let (px, py) = p;
        let (ax, ay) = a;
        let (bx, by) = b;
        let (cx, cy) = c;

        let v0 = (cx - ax, cy - ay);
        let v1 = (bx - ax, by - ay);
        let v2 = (px - ax, py - ay);

        let dot00 = v0.0 * v0.0 + v0.1 * v0.1;
        let dot01 = v0.0 * v1.0 + v0.1 * v1.1;
        let dot02 = v0.0 * v2.0 + v0.1 * v2.1;
        let dot11 = v1.0 * v1.0 + v1.1 * v1.1;
        let dot12 = v1.0 * v2.0 + v1.1 * v2.1;

        let denom = dot00 * dot11 - dot01 * dot01;
        if denom.abs() < f32::EPSILON {
            return false; // zdegenerowany trójkąt
        }
        let inv_denom = 1.0 / denom;
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        (u >= 0.0) && (v >= 0.0) && (u + v <= 1.0)
    }

    pub fn closest_point_on_segment(&self, p: (f32, f32), a: (f32, f32), b: (f32, f32)) -> (f32, f32) {
        let (px, py) = p;
        let (ax, ay) = a;
        let (bx, by) = b;

        let ab = (bx - ax, by - ay);
        let ap = (px - ax, py - ay);

        let ab_len2 = ab.0 * ab.0 + ab.1 * ab.1;
        if ab_len2 == 0.0 {
            return a; // odcinek zdegenerowany
        }

        let t = ((ap.0 * ab.0 + ap.1 * ab.1) / ab_len2).clamp(0.0, 1.0);
        (ax + t * ab.0, ay + t * ab.1)
    }

    pub fn dist(&self, a: (f32, f32), b: (f32, f32)) -> f32 {
        let dx = a.0 - b.0;
        let dy = a.1 - b.1;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn clamp_xy_to_srgb_gamut(&self, x: f32, y: f32) -> (f32, f32) {
        let r = (0.64f32, 0.33f32);
        let g = (0.30f32, 0.60f32);
        let b = (0.15f32, 0.06f32);
        let p = (x, y);

        if self.point_in_triangle(p, r, g, b) {
            return p;
        }

        let c1 = self.closest_point_on_segment(p, r, g);
        let c2 = self.closest_point_on_segment(p, g, b);
        let c3 = self.closest_point_on_segment(p, b, r);

        let mut best = c1;
        let mut best_d = self.dist(p, c1);

        let d2 = self.dist(p, c2);
        if d2 < best_d {
            best = c2;
            best_d = d2;
        }

        let d3 = self.dist(p, c3);
        if d3 < best_d {
            best = c3;
        }

        best
    }

    fn pick_and_load_bg(&mut self, ctx: &egui::Context) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Obrazy", &["png", "jpg", "jpeg"])
            .pick_file()
        {
            match image::open(&path) {
                Ok(img) => {
                    let rgba = img.to_rgba8();
                    let (w, h) = rgba.dimensions();
                    let pixels = rgba.into_raw();
                    let color_image = egui::ColorImage::from_rgba_unmultiplied(
                        [w as usize, h as usize],
                        &pixels,
                    );
                    let tex = ctx.load_texture(
                        format!("bg_{}", path.display()),
                        color_image,
                        egui::TextureOptions::LINEAR,
                    );
                    self.bg_texture = Some(tex);
                }
                Err(err) => {
                    eprintln!("Błąd wczytywania obrazu: {err}");
                }
            }
        }
    }
}

impl MyApp {
    fn save_state(&self, path: &str) -> Result<(), std::io::Error> {
        let state = MyAppState {
            points: self.points.clone(),
            wavelengths: self.wavelengths.clone(),
            current_xy: self.current_xy,
            current_rgb: self.current_rgb,
            control_points: self.control_points.clone(),
            max_points: self.max_points,
            xyz_samples: self.xyz_samples.clone(),
            bg_mode: self.bg_mode,
            bg_opacity: self.bg_opacity,
            curve: self.curve,
        };

        let json = serde_json::to_string_pretty(&state).unwrap();
        std::fs::write(path, json)
    }

    fn load_state(&mut self, path: &str) -> Result<(), std::io::Error> {
        let data = std::fs::read_to_string(path)?;
        let state: MyAppState = serde_json::from_str(&data).unwrap();

        self.points = state.points;
        self.wavelengths = state.wavelengths;
        self.current_xy = state.current_xy;
        self.current_rgb = state.current_rgb;
        self.control_points = state.control_points;
        self.max_points = state.max_points;
        self.xyz_samples = state.xyz_samples;
        self.bg_mode = state.bg_mode;
        self.bg_opacity = state.bg_opacity;
        self.curve = state.curve;

        Ok(())
    }
}
