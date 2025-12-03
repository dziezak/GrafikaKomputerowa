
pub fn xyz_to_xy(x: f32, y: f32, z: f32) -> (f32, f32) {
    let sum = x + y + z;
    if sum == 0.0 {
        return (0.0, 0.0);
    }
    (x / sum, y / sum)
}


pub fn xyz_to_srgb(X: f32, Y: f32, Z: f32) -> (u8, u8, u8) {
    let r_lin = 3.2406 * X - 1.5372 * Y - 0.4986 * Z;
    let g_lin = -0.9689 * X + 1.8758 * Y + 0.0415 * Z;
    let b_lin = 0.0557 * X - 0.2040 * Y + 1.0570 * Z;

    fn gamma_correct(c: f32) -> f32 {
        if c <= 0.0031308 {
            12.92 * c
        } else {
            1.055 * c.powf(1.0 / 2.4) - 0.055
        }
    }

    let r = gamma_correct(r_lin.max(0.0).min(1.0));
    let g = gamma_correct(g_lin.max(0.0).min(1.0));
    let b = gamma_correct(b_lin.max(0.0).min(1.0));

    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}
