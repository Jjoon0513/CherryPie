
mod cherryblossom;
mod mainwindow;

use eframe::egui;
use std::fs;
use std::sync::Arc;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CherryPie",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(mainwindow::MainWindow::default()))
        }))
}
fn setup_custom_fonts(ctx: &egui::Context) {
    use egui::{FontData, FontDefinitions, FontFamily};
    use std::collections::BTreeMap;

    let mut fonts = FontDefinitions {
        font_data: BTreeMap::new(),
        families: BTreeMap::new(),
    };

    // 1. JetBrainsMono (영어용)
    if let Ok(jetbrains) = fs::read("fonts/JetBrainsMono-Regular.ttf") {
        fonts.font_data.insert(
            "english".to_owned(),
            Arc::from(FontData::from_owned(jetbrains)),
        );
    }

    // 2. GothicA1 (한글용)
    if let Ok(korean) = fs::read("fonts/NotoSansKR-Regular.ttf") {
        fonts.font_data.insert(
            "korean".to_owned(),
            Arc::from(FontData::from_owned(korean)),
        );
    }

    // 3. Proportional font priority
    fonts
        .families
        .insert(FontFamily::Proportional, vec!["english".to_owned(), "korean".to_owned()]);

    // 4. Monospace도 설정할 수 있음 (코드용)
    fonts
        .families
        .insert(FontFamily::Monospace, vec!["english".to_owned(), "korean".to_owned()]);

    ctx.set_fonts(fonts);
}
