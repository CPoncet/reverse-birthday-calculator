use chrono::{Date, Utc};
use eframe::{App, egui};
use reverse_birthday_calculator::{AgeInput, calc_birthday};
use egui_extras::DatePickerButton;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Reverse birthday calculator",
        options,
        Box::new(|_cc| Box::new(AppData::default()))
    )
}

struct AppData {
    age: AgeInput,
    date: String,
    result: String
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            age: AgeInput {
                years: "27".to_string(),
                months: "4".to_string(),
                days: "12".to_string()
            },
            date: String::new(),
            result: String::new()
        }
    }
}

impl App for AppData {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Reverse birthday calculator");
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


            if ui.button("Ok").clicked() {
                self.result = calc_birthday(&self.age, &self.date).to_string()
            }

            ui.label(format!("Date '{}'", self.result));
        });
    }
}