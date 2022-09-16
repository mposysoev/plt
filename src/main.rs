use egui::{
    self,
    plot::{Line, Plot, PlotPoints},
};
use std::env;
use std::{fs::File, io::Read};

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

fn get_data_from_file(path_to_file: &str) -> Vec<[f64; 2]> {
    let mut data = File::open(path_to_file).unwrap();
    let mut contents = String::new();
    data.read_to_string(&mut contents).unwrap();

    let lines = contents.lines();

    let mut values = Vec::new();

    for line in lines {
        if line.starts_with('#') {
            continue;
        } else {
            let mut iter = line.split_ascii_whitespace();
            let x: f64 = iter.next().unwrap().parse().unwrap();
            let y: f64 = iter.next().unwrap().parse().unwrap();
            values.push([x, y]);
        }
    }
    values
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file_data = get_data_from_file(file_name);

    let app = App::new(file_data);
    let options = eframe::NativeOptions::default();
    eframe::run_native(file_name, options, Box::new(|_cc| Box::new(app)));
}
