use eframe::egui::CursorIcon::Default;
use eframe::egui::TextStyle::Body;
use eframe::egui::*;
use eframe::egui::{
    self, Button, Color32, FontDefinitions, FontFamily, Hyperlink, Label, Layout, Separator,
    TopBottomPanel,
};
use egui::menu::bar;
use std::borrow::Cow;
use std::fmt::format;

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct HeadlinesConfig {
    pub dark_mode: bool,
}

pub struct Samachar {
    articles: Vec<NewsCardData>,
    pub config: HeadlinesConfig,
}

struct NewsCardData {
    title: String,
    description: String,
    url: String,
}

impl HeadlinesConfig {
    fn new() -> Self {
        Self { dark_mode: true }
    }
}

impl Samachar {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Samachar {
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

        font_def
            .families
            .insert(FontFamily::Proportional, vec!["RobotoSlab".to_owned()]);

        cc.egui_ctx.set_fonts(font_def);
        Samachar {
            articles: Vec::from_iter(iter),
            config: HeadlinesConfig::new(),
        }
    }

    pub fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
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
            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                ui.add(Hyperlink::from_label_and_url("read more.", &article.url));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub(crate) fn render_top_panel(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // Define the top panel
        TopBottomPanel::top("top_panel").show(ctx, |ui: &mut Ui| {
            bar(ui, |ui: &mut Ui| {
                // logo
                ui.with_layout(Layout::left_to_right(Align::Center), |ui: &mut Ui| {
                    ui.add(Label::new(format!("S")));
                });
                // Container
                ui.with_layout(Layout::right_to_left(Align::Center), |ui: &mut Ui| {
                    let close_button = ui.add(Button::new("Close")); // ICons are not currently working
                    let refers_button = ui.add(Button::new("Render"));
                    let theme_button = ui.add(Button::new("Theme"));

                    if theme_button.clicked() {
                        self.config.dark_mode = !self.config.dark_mode;
                    }

                    if (close_button.clicked()) {
                        frame.quit();
                    }
                })
            })
        });
    }
}
