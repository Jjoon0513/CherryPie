
mod cherryblossom;

use eframe::egui;
use std::fs;
use std::sync::Arc;
use egui_code_editor::{self, highlighting::Token, CodeEditor, ColorTheme, Syntax};
use crate::cherryblossom::CherryBlossomSyntax;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CherryPie",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(MainWindow::default()))
        }))
}

fn setup_custom_fonts(ctx: &egui::Context) {
    use egui::FontData;
    use egui::FontDefinitions;
    use egui::FontFamily::Proportional;

    let mut fonts = FontDefinitions::default();

    if let Ok(font_data) = fs::read("fonts/GothicA1-Bold.ttf") {
        fonts.font_data.insert(
            "my_font".to_owned(),
            Arc::from(FontData::from_owned(font_data)),
        );

        fonts
            .families
            .get_mut(&Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());

        ctx.set_fonts(fonts);
    }
}



#[derive(Default)]
struct MainWindow{
    code: String
}


impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        ctx.input(|input| {
            for event in &input.events {
                if let egui::Event::Text(text) = event {
                    if text == "(" {
                        self.code.push_str(")");
                    }
                    if text == "{" {
                        self.code.push_str("}");
                    }
                }
            }
        });



        egui::CentralPanel::default().show(ctx, |ui| {
            CodeEditor::default()
                .id_source("code editor")
                .with_rows(12)
                .with_fontsize(14.0)
                .with_theme(ColorTheme::AYU_DARK)
                .with_syntax(Syntax::cherry_blossom())
                .with_numlines(true)
                .show(ui, &mut self.code);
        });
    }
}
