mod data_loader;
mod cie;
mod renderer;
mod bezier;

use eframe::egui;
use data_loader::load_xyz_data;
use cie::xyz_to_xy;
use renderer::draw_chromaticity;
use crate::bezier::evaluate_curve;
use crate::cie::xyz_to_srgb;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CIE Chromaticity Diagram",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}


struct MyApp {
    // Lewy panel: podkowa
    points: Vec<(f32, f32)>,
    wavelengths: Vec<f32>,
    current_xy: (f32, f32),
    current_rbg: (u8, u8, u8),

    // Prawy panel: krzywa Béziera (rozklad widmowy)
    control_points: Vec<(f32, f32)>, // (λ in nm, I in [0..1.8])
    dragging_idx: Option<usize>,      // który punkt aktualnie przesuwamy
    max_points: usize,                // ograniczenie liczby punktów

    xyz_samples: Vec<(f32, f32, f32, f32)>,
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
            current_rbg: (0, 0, 0),
            control_points: vec![(400.0, 0.6), (500.0, 1.2), (620.0, 0.8)],
            dragging_idx: None,
            max_points: 6,
            xyz_samples,
        }
    }
}

impl MyApp {


    fn compute_color_from_bezier(&self) -> ((f32, f32), (u8, u8, u8)) {
        let mut X = 0.0;
        let mut Y = 0.0;
        let mut Z = 0.0;
        let mut total_intensity = 0.0;

        if let Ok(xyz_data) = load_xyz_data("src/assets/data.txt") {
            for (wl, x_bar, y_bar, z_bar) in xyz_data {
                if wl >= 380.0 && wl <= 700.0 {
                    let intensity = bezier::evaluate_curve(&self.control_points, wl);
                    total_intensity += intensity;

                    X += intensity * x_bar;
                    Y += intensity * y_bar;
                    Z += intensity * z_bar;
                }
            }
        }

        if total_intensity > 0.0 {
            X /= total_intensity;
            Y /= total_intensity;
            Z /= total_intensity;
        }

        let sum = X + Y + Z;
        if sum > 0.0 {
            X /= sum;
            Y /= sum;
            Z /= sum;
        }

        let xy = (X, Y);
        let rgb = cie::xyz_to_srgb(X, Y, Z);
        (xy, rgb)
    }



    fn compute_color_from_curve(&self) -> ((f32, f32), (u8, u8, u8)) {
        let mut X = 0.0;
        let mut Y = 0.0;
        let mut Z = 0.0;
        let mut total_intensity = 0.0;

        for (wl, x_bar, y_bar, z_bar) in &self.xyz_samples {
            if *wl >= 380.0 && *wl <= 700.0 {
                let p = evaluate_curve(&self.control_points, *wl);
                total_intensity += p;
                X += p * *x_bar;
                Y += p * *y_bar;
                Z += p * *z_bar;
            }
        }

        if total_intensity > 0.0 {
            X /= total_intensity;
            Y /= total_intensity;
            Z /= total_intensity;
        }

        let sum = X + Y + Z;
        let (x_chr, y_chr) = if sum > 0.0 { (X / sum, Y / sum) } else { (0.0, 0.0) };

        let rgb = xyz_to_srgb(X, Y, Z);
        ((x_chr, y_chr), rgb)
    }


}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Najpierw przelicz bieżący punkt z krzywej widmowej


        let (xy, rgb) = self.compute_color_from_curve();
        self.current_xy = xy;
        self.current_rbg = rgb;

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            renderer::draw_chromaticity(ui, &self.points, &self.wavelengths, self.current_xy, self.current_rbg);
            ui.label(format!("xy: ({:.4}, {:.4}) | RGB: ({}, {}, {})", xy.0, xy.1, rgb.0, rgb.1, rgb.2));
        });


        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Limit punktów Béziera:");
                ui.add(egui::DragValue::new(&mut self.max_points).clamp_range(1..=16));
                ui.small("Kliknięciem dodajesz punkty; przeciąganiem przesuwasz.");
            });
            ui.separator();

            bezier::draw_bezier_interactive(
                ui,
                &mut self.control_points,
                &mut self.dragging_idx,
                self.max_points,
            );
        });
    }
}
