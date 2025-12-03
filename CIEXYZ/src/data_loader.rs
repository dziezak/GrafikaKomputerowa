
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_xyz_data(path: &str) -> Result<Vec<(f32, f32, f32, f32)>, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 4 {
            let wl = parts[0].parse::<f32>().unwrap();
            let x = parts[1].parse::<f32>().unwrap();
            let y = parts[2].parse::<f32>().unwrap();
            let z = parts[3].parse::<f32>().unwrap();
            data.push((wl, x, y, z));
        }
    }
    Ok(data)
}
