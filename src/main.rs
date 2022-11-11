use eframe::egui::*;
use std::borrow::Cow;
use std::fmt::format;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Samachar",
        native_options,
        Box::new(|cc| Box::new(Samachar::new(cc))),
    );
}

#[derive(Default)]
struct Samachar {
    articles: Vec<NewsCardData>,
}

struct NewsCardData {
    title: String,
    description: String,
    url: String,
}

impl Samachar {
    fn new(cc: &eframe::CreationContext<'_>) -> Samachar {
        Self::default();

        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title: {}", a),
            description: format!("description: {}", a),
            url: format!("https://example.com/{}", a),
        });

        let mut font_def = FontDefinitions::default();

        font_def.font_data.insert(
            "RobotoSlab".to_owned(),
            FontData::from_static(include_bytes!("../RobotoSlab-VariableFont_wght.ttf")).tweak(
                FontTweak {
                    scale: 2.0,
                    y_offset_factor: 0.0,
                    y_offset: 0.0,
                },
            ),
        );

        font_def.families.insert(
            FontFamily::Proportional,
            vec![
                "RobotoSlab".to_owned()
            ],
        );

        cc.egui_ctx.set_fonts(font_def);
        Samachar {
            articles: Vec::from_iter(iter),
        }
    }
}

impl eframe::App for Samachar {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Samachar: The Headline");

            ScrollArea::auto_shrink(ScrollArea::new([true; 2]), [false; 2]).show(ui, |ui| {
                for article in &self.articles {
                    ui.label(&article.title);
                    ui.label(&article.description);
                    ui.label(&article.url);
                }
            })
        });
    }
}
