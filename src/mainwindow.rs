use eframe::egui;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use crate::cherryblossom::CherryBlossomSyntax;
use std::fs;
use std::path::PathBuf;
use rfd::FileDialog;

#[derive(Default)]
pub struct MainWindow{
    code: String,
    cursor_pos: usize,
    filepath: PathBuf,
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
                                            let replacement = format!("\n{}{}", "\t".repeat(indent - 1), "}");
                                            self.code.replace_range(byte_index..=byte_index, &replacement);
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
                ui.menu_button("파일", |ui| {
                    if ui.button("열기").clicked() {
                        if let Some(path) = FileDialog::new().pick_file() {
                            self.filepath = path.clone();
                            match fs::read_to_string(&path) {
                                Ok(content) => {
                                    self.code = content;
                                }
                                Err(e) => {
                                    eprintln!("파일 읽기 오류: {}", e);
                                }
                            }

                        }
                        ui.close_menu();
                    }
                    if ui.button("저장").clicked() {
                        match fs::write(self.filepath.clone(), self.code.clone()) {
                            Ok(_) => println!("저장 성공!"),
                            Err(e) => eprintln!("저장 실패: {}", e),
                        }
                        ui.close_menu();
                    }
                    if ui.button("다른 이름으로 저장").clicked() {
                        if let Some(path) = FileDialog::new().save_file() {
                            match fs::write(path, self.code.clone()) {
                                Ok(_) => println!("저장 성공!"),
                                Err(e) => eprintln!("저장 실패: {}", e),
                            }
                        }
                        ui.close_menu();
                    }

                    ui.separator();

                    if ui.button("설정").clicked(){
                        ui.close_menu();
                    }
                });

                ui.menu_button("에딧", |ui| {
                    if ui.button("뒤로가기").clicked() {
                        println!("Undo clicked");
                        ui.close_menu();
                    }
                });

                ui.menu_button("빌드", |ui| {
                    if ui.button("프로젝트 빌드").clicked() {
                        println!("Undo clicked");
                        ui.close_menu();
                    }
                });

                ui.menu_button("실행", |ui| {
                    if ui.button("프로젝트 실행").clicked() {
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
