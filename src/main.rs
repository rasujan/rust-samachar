mod headlines;

use eframe::egui::panel::TopBottomSide::Top;
use eframe::egui::{
    self, Button, CentralPanel, Color32, Context, FontDefinitions, FontFamily, Hyperlink, Label,
    Layout, ScrollArea, Separator, TopBottomPanel,
};
use headlines::{Samachar, PADDING};

impl eframe::App for Samachar {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::auto_shrink(ScrollArea::new([true; 2]), [false; 2])
                .show(ui, |ui| self.render_news_cards(ui));
            render_footer(ctx);
        });
    }
}

fn render_header(ui: &mut eframe::egui::Ui) {
    ui.vertical_centered(|ui| ui.heading("Samachar"));

    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20 as f32);
    ui.add(sep);
}

fn render_footer(ctx: &Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui: &mut eframe::egui::Ui| {
        ui.vertical_centered(|ui: &mut eframe::egui::Ui| {
            ui.add_space(10.00);
            // source of the newsletter
            ui.add(Label::new(format!("API source: newsapi.org")));
            // made with eGUI
            ui.add(Label::new(format!("Made with eGUI")));
            ui.add_space(10.00);
        })
    });
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Samachar",
        native_options,
        Box::new(|cc| Box::new(Samachar::new(cc))),
    );
}
