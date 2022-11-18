mod headlines;

use eframe::egui::{self,CentralPanel, ScrollArea, Context, Button, Color32, FontDefinitions, FontFamily, Hyperlink, Label, Layout, Separator, TopBottomPanel};
use headlines::{Samachar, PADDING};

impl eframe::App for Samachar {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Samachar: The Headline");

            ScrollArea::auto_shrink(ScrollArea::new([true; 2]), [false; 2]).show(ui, |ui| {
                self.render_news_cards(ui)
            })
        });
    }


}


fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Samachar",
        native_options,
        Box::new(|cc| Box::new(Samachar::new(cc))),
    );
}



