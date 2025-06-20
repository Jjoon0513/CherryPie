use std::fs;
use std::process::Command;
use eframe::egui::Ui;
use rfd::FileDialog;
use crate::mainwindow::MainWindow;

pub fn manubar(main_window: &mut MainWindow, ui: &mut Ui) {
    ui.menu_button("파일", |ui| {
        if ui.button("열기").clicked() {
            if let Some(path) = FileDialog::new().add_filter("CB 파일", &["cb"]).pick_file() {
                main_window.filepath = path.clone();
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        main_window.code = content;
                    }
                    Err(e) => {
                        eprintln!("파일 읽기 오류: {}", e);
                    }
                }

            }
            ui.close_menu();
        }


        if ui.button("다른 이름으로 저장").clicked() {
            if let Some(path) = FileDialog::new().save_file() {
                match fs::write(path, main_window.code.clone()) {
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
            if let Err(e) = fs::write(main_window.filepath.clone(), main_window.code.clone()) {
                main_window.console = format!("파일 저장 실패: {}", e);
                ui.close_menu();
                return;
            }

            let path = main_window.filepath.clone();
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            let parent = path.parent().unwrap();

            let output = Command::new("cherry")
                .arg(&filename)
                .current_dir(parent)
                .output();

            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    main_window.console = format!(
                        "[stdout]\n{}\n[stderr]\n{}",
                        stdout.trim(),
                        stderr.trim()
                    );
                }
                Err(e) => {
                    main_window.console = format!("실행 중 오류 발생: {}", e);
                }
            };

            println!("{}", main_window.console);

            ui.close_menu();
        }
    });
}