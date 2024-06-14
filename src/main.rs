use egui::{self};
use egui_plot::{Line, Plot, PlotPoints};
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Clone)]
struct App {
    data: Vec<[f64; 2]>,
}

impl App {
    fn new(file_data: Vec<[f64; 2]>) -> Self {
        Self { data: file_data }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let points_data = self.data.clone();
        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("data");
            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::new(points_data)));
            });
        });
    }
}

fn get_data_from_file(path_to_file: &str) -> io::Result<Vec<[f64; 2]>> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);
    let mut values = Vec::new();
    let mut line_number = 0.0;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') || line.starts_with('%') || line.starts_with('/') {
            continue;
        }
        let mut iter = line.split_ascii_whitespace();
        let x: f64;
        let y: f64;

        if let Some(first_value) = iter.next() {
            let first_value: f64 = first_value.parse().unwrap_or_default();
            if let Some(second_value) = iter.next() {
                // If there's a second value, use the first as x and the second as y
                x = first_value;
                y = second_value.parse().unwrap_or_default();
            } else {
                // If there's only one value, use line number as x and the value as y
                x = line_number;
                y = first_value;
            }
            values.push([x, y]);
            line_number += 1.0;
        }
    }
    Ok(values)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_name = &args[1];
    let app_name = format!("Plot for {}", file_name);
    
    match get_data_from_file(file_name) {
        Ok(file_data) => {
            let app = App::new(file_data);
            let options = eframe::NativeOptions::default();
            let _ = eframe::run_native(&app_name, options, Box::new(|_cc| Box::new(app)));
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}
