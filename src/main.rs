mod headlines;

use eframe::egui::panel::TopBottomSide::Top;
use eframe::egui::{
    self, Button, CentralPanel, Color32, Context, FontDefinitions, FontFamily, Hyperlink, Label,
    Layout, ScrollArea, Separator, TopBottomPanel, Ui, Visuals,
};
use headlines::{Samachar, PADDING};

impl eframe::App for Samachar {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        if self.config.dark_mode == true {
            ctx.set_visuals(Visuals::dark())
        } else {
            ctx.set_visuals(Visuals::light())
        }

        self.render_top_panel(ctx, frame);

        self.render_config(ctx);

        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::auto_shrink(ScrollArea::new([true; 2]), [false; 2])
                .show(ui, |ui| self.render_news_cards(ui));
            render_footer(ctx);
        });
    }
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| ui.heading("Samachar"));

    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20 as f32);
    ui.add(sep);
}

fn render_footer(ctx: &Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui: &mut Ui| {
        ui.vertical_centered(|ui: &mut Ui| {
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
    tracing_subscriber::fmt::init();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Samachar",
        native_options,
        Box::new(|cc| Box::new(Samachar::new(cc))),
    );
}
