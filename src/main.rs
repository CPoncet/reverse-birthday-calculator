#![windows_subsystem = "windows"]

use eframe::{egui, App};
use reverse_date_calculator::{calc_birthday, AgeInput};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Reverse date calculator",
        options,
        Box::new(|_cc| Box::new(AppData::default())),
    )
}

struct AppData {
    age: AgeInput,
    date: String,
    leap: bool,
    result: String,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            age: AgeInput {
                years: String::new(),
                months: String::new(),
                days: String::new(),
            },
            leap: true,
            date: String::new(),
            result: String::new(),
        }
    }
}

impl App for AppData {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Reverse date calculator");
            ui.horizontal(|ui| {
                ui.heading("Date: ");

                ui.label("YYYY/MM/DD");
                ui.text_edit_singleline(&mut self.date);
            });
            ui.horizontal(|ui| {
                ui.heading("Age: ");
            });
            ui.horizontal(|ui| {
                ui.label("Years: ");
                ui.text_edit_singleline(&mut self.age.years);
            });

            ui.horizontal(|ui| {
                ui.label("Months: ");
                ui.text_edit_singleline(&mut self.age.months);
            });

            ui.horizontal(|ui| {
                ui.label("Days: ");
                ui.text_edit_singleline(&mut self.age.days);
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.leap, "Exclude leap years");
            });

            if ui.button("Ok").clicked() {
                self.result = match calc_birthday(&self.age, &self.date, &self.leap) {
                    Ok(date) => date.to_string(),
                    Err(err) => err,
                }
            }

            ui.label(format!("Date '{}'", self.result));
        });
    }
}
