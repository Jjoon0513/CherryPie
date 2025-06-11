
mod cherryblossom;

use eframe::egui;
use std::fs;
use std::sync::Arc;
use egui_code_editor::{self, CodeEditor, ColorTheme, Syntax};
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
    code: String,
    cursor_pos: usize,
}


impl eframe::App for MainWindow {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        ctx.input(|input| {
            for event in &input.events {
                match event {
                    egui::Event::Text(text) => {
                        if text == "(" {
                            self.code.insert_str(self.cursor_pos, ")")
                        }
                        if text == "{" {
                            self.code.insert_str(self.cursor_pos, "}")
                        }
                        if text == "[" {
                            self.code.insert_str(self.cursor_pos, "]")
                        }
                        if text == "\"" {
                            self.code.insert_str(self.cursor_pos, "\"")
                        }
                        if text == "\'" {
                            self.code.insert_str(self.cursor_pos, "\'")
                        }
                    }
                    egui::Event::Key {
                        key: egui::Key::Backspace,
                        pressed: true,
                        repeat: false,
                        ..
                    } => {
                        let mut chars: Vec<char> = self.code.chars().collect();

                        if self.cursor_pos > 0 {
                            if let Some(ch) = self.code.chars().nth(self.cursor_pos - 1) {
                                if ch == '(' {
                                    if self.cursor_pos < chars.len() && chars[self.cursor_pos] == ')' {
                                        chars.remove(self.cursor_pos);
                                        self.code = chars.into_iter().collect();
                                    }
                                } else if ch == '[' {
                                    if self.cursor_pos < chars.len() && chars[self.cursor_pos] == ']' {
                                        chars.remove(self.cursor_pos);
                                        self.code = chars.into_iter().collect();
                                    }
                                } else if ch == '{' {
                                    if self.cursor_pos < chars.len() && chars[self.cursor_pos] == '}' {
                                        chars.remove(self.cursor_pos);
                                        self.code = chars.into_iter().collect();
                                    }
                                } else if ch == '\'' {
                                    if self.cursor_pos < chars.len() && chars[self.cursor_pos] == '\'' {
                                        chars.remove(self.cursor_pos);
                                        self.code = chars.into_iter().collect();
                                    }
                                } else if ch == '\"' {
                                    if self.cursor_pos < chars.len() && chars[self.cursor_pos] == '\"' {
                                        chars.remove(self.cursor_pos);
                                        self.code = chars.into_iter().collect();
                                    }
                                }
                            }

                        }
                    }
                    egui::Event::Key {
                        key: egui::Key::Enter,
                        pressed: true,
                        repeat: false,
                        ..
                    } => {



                        if self.cursor_pos > 0 {
                            let chars: Vec<char> = self.code.chars().collect();

                            let char_pos = self.code[..self.cursor_pos].chars().count();

                            if let Some(&ch) = chars.get(char_pos - 1) {
                                if ch == '{' {
                                    let codesliced: Vec<char> = chars[..char_pos].to_vec();

                                    let open = codesliced.iter().filter(|&&c| c == '{').count();
                                    let close = codesliced.iter().filter(|&&c| c == '}').count();
                                    let indent = open.saturating_sub(close);

                                    if let Some(&next_ch) = chars.get(char_pos) {
                                        if next_ch == '}' {

                                            let byte_index = self.code.char_indices().nth(char_pos).map(|(i, _)| i).unwrap();
                                            println!("open count: {}",open);
                                            println!("close count: {}",close);
                                            println!("indent count: {}",indent);
                                            self.code.insert_str(byte_index - 1, &format!("{}", "\t".repeat(indent)));
                                            self.code.insert_str(byte_index, &format!("{}", "}"))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }


            }
        });

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        println!("Open clicked");
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        println!("Save clicked");
                        ui.close_menu();
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        println!("Undo clicked");
                        ui.close_menu();
                    }
                });
            });
        });


        egui::CentralPanel::default().show(ctx, |ui| {
            let editor = CodeEditor::default()
                .id_source("code editor")
                .vscroll(true)
                .with_rows(100)
                .with_fontsize(14.0)
                .with_theme(ColorTheme::AYU_DARK)
                .with_syntax(Syntax::cherry_blossom())
                .with_numlines(true)
                .show(ui, &mut self.code);

            if editor.response.has_focus(){
                self.cursor_pos = editor.cursor_range.unwrap().primary.ccursor.index
            }

        });
    }
}
