use eframe::egui;
use eframe::egui::InputState;
use crate::mainwindow::MainWindow;

pub fn cherrypie_input_system(main_window: &mut MainWindow, inputstate: &InputState){
    for event in &inputstate.events {
        match event {
            egui::Event::Text(text) => {
                if text == "(" {
                    main_window.code.insert_str(main_window.cursor_pos, ")")
                }
                if text == "{" {
                    main_window.code.insert_str(main_window.cursor_pos, "}")
                }
                if text == "[" {
                    main_window.code.insert_str(main_window.cursor_pos, "]")
                }
                if text == "\"" {
                    main_window.code.insert_str(main_window.cursor_pos, "\"")
                }
                if text == "\'" {
                    main_window.code.insert_str(main_window.cursor_pos, "\'")
                }
            }
            egui::Event::Key {
                key: egui::Key::Backspace,
                pressed: true,
                repeat: false,
                ..
            } => {
                let mut chars: Vec<char> = main_window.code.chars().collect();

                if main_window.cursor_pos > 0 {
                    if let Some(ch) = main_window.code.chars().nth(main_window.cursor_pos - 1) {
                        if ch == '(' {
                            if main_window.cursor_pos < chars.len() && chars[main_window.cursor_pos] == ')' {
                                chars.remove(main_window.cursor_pos);
                                main_window.code = chars.into_iter().collect();
                            }
                        } else if ch == '[' {
                            if main_window.cursor_pos < chars.len() && chars[main_window.cursor_pos] == ']' {
                                chars.remove(main_window.cursor_pos);
                                main_window.code = chars.into_iter().collect();
                            }
                        } else if ch == '{' {
                            if main_window.cursor_pos < chars.len() && chars[main_window.cursor_pos] == '}' {
                                chars.remove(main_window.cursor_pos);
                                main_window.code = chars.into_iter().collect();
                            }
                        } else if ch == '\'' {
                            if main_window.cursor_pos < chars.len() && chars[main_window.cursor_pos] == '\'' {
                                chars.remove(main_window.cursor_pos);
                                main_window.code = chars.into_iter().collect();
                            }
                        } else if ch == '\"' {
                            if main_window.cursor_pos < chars.len() && chars[main_window.cursor_pos] == '\"' {
                                chars.remove(main_window.cursor_pos);
                                main_window.code = chars.into_iter().collect();
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



                if main_window.cursor_pos > 0 {
                    let chars: Vec<char> = main_window.code.chars().collect();

                    let char_pos = main_window.code[..main_window.cursor_pos].chars().count();

                    if let Some(&ch) = chars.get(char_pos - 1) {
                        if ch == '{' {
                            let codesliced: Vec<char> = chars[..char_pos].to_vec();

                            let open = codesliced.iter().filter(|&&c| c == '{').count();
                            let close = codesliced.iter().filter(|&&c| c == '}').count();
                            let indent = open.saturating_sub(close);

                            if let Some(&next_ch) = chars.get(char_pos) {
                                if next_ch == '}' {
                                    let byte_index = main_window.code.char_indices().nth(char_pos).map(|(i, _)| i).unwrap();
                                    let replacement = format!("\n{}{}", "\t".repeat(indent - 1), "}");
                                    main_window.code.replace_range(byte_index..=byte_index, &replacement);
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }


    }

}