use eframe::{
    self,
    egui::{self},
};
use regex::Regex;
#[derive(Default)]
struct MyApp {
    regex: String,
    text1: String,
    text2: String,
    match1: String,
    match2: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Enter Your Data");

            ui.label("Regex");
            let response = ui.add(egui::TextEdit::singleline(&mut self.regex));
            if response.changed() {
                apply_regex(
                    &self.regex,
                    &mut self.match1,
                    &mut self.match2,
                    ctx,
                    &mut self.text1,
                    &mut self.text2,
                );
            }

            ui.label("Second text:");
            let first_test = ui.add(egui::TextEdit::singleline(&mut self.text1));
            if first_test.changed() {
                {
                    apply_regex(
                        &self.regex,
                        &mut self.match1,
                        &mut self.match2,
                        ctx,
                        &mut self.text1,
                        &mut self.text2,
                    );
                }
            }
            ui.add(egui::Label::new(&self.match1));

            ui.label("Third text:");
            let second_test = ui.add(egui::TextEdit::singleline(&mut self.text2));
            if second_test.changed() {
                {
                    apply_regex(
                        &self.regex,
                        &mut self.match1,
                        &mut self.match2,
                        ctx,
                        &mut self.text1,
                        &mut self.text2,
                    );
                }
            }
            ui.add(egui::Label::new(&self.match2));

            ui.separator();
        });
    }
}

fn apply_regex(
    regex: &str,
    match1: &mut String,
    match2: &mut String,
    ctx: &egui::Context,
    text1: &mut String,
    text2: &mut String,
) {
    let re = match Regex::new(regex) {
        Err(_) => return,
        Ok(t) => t,
    };
    let result_match_one = re.is_match(text1);
    let result_match_two = re.is_match(text2);
    match result_match_one {
        true => {
            match1.clear();
            match1.push_str("Correto");
            ctx.request_repaint();
        }
        false => {
            match1.clear();
            match1.push_str("Incorreto");
            ctx.request_repaint();
        }
    }
    match result_match_two {
        true => {
            match2.clear();
            match2.push_str("Correto");
        }
        false => {
            match2.clear();
            match2.push_str("Incorreto");
        }
    }
    ctx.request_repaint();
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Three Text Inputs",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
