use eframe::egui::*;
use std::borrow::Cow;
use std::fmt::format;
use eframe::egui::{self, Button, Color32, FontDefinitions, FontFamily, Hyperlink, Label, Layout, Separator, TopBottomPanel};

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);


pub struct Samachar {
    articles: Vec<NewsCardData>,
}

struct NewsCardData {
    title: String,
    description: String,
    url: String,
}

impl Samachar {
   pub  fn new(cc: &eframe::CreationContext<'_>) -> Samachar {
        // Self::default();

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

   pub fn render_news_cards(&self, ui: &mut eframe::egui::Ui){
        for article in &self.articles {
            // Render Title
            ui.add_space(PADDING);
            let title = format!("{}", article.title);
            ui.colored_label(WHITE, title);

            // Render Description
            ui.add_space(PADDING);
            let description = Label::new(&article.description);
            ui.add(description);


            // render hyperlinks
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.add(Hyperlink::from_label_and_url("read more.", &article.url));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }
}