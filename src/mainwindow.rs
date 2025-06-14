use eframe::egui;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use crate::cherryblossom::CherryBlossomSyntax;
use std::fs;
use std::path::PathBuf;
use rfd::FileDialog;
use std::process::Command;
use crate::util::{system, menubar};
use system::cherrypie_input_system;
use menubar::manubar;


#[derive(Default)]
pub struct MainWindow{
    pub code: String,
    pub cursor_pos: usize,
    pub filepath: PathBuf,
}


impl eframe::App for MainWindow {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        ctx.input(|input| {
            cherrypie_input_system(self, input)
        });

        egui::SidePanel::left("project_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("📁 프로젝트 구조");
                // 여기에 파일 목록, 탐색기 등 구현
            });

        egui::SidePanel::right("right_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("ℹ️ 유틸리티");

            });

        egui::TopBottomPanel::bottom("console_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("🖨️ 출력 / 콘솔");
                // 컴파일 결과, 로그, 오류 메시지 등
            });

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                manubar(self, ui);
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


