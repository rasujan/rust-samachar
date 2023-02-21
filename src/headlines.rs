// use eframe::egui::CursorIcon::Default;
// use eframe::egui::TextStyle::Body;
use eframe::egui::*;
use eframe::egui::{
    self, Button, Color32, FontDefinitions, FontFamily, Hyperlink, Label, Layout, Separator,
    TopBottomPanel, Window,
};
use egui::menu::bar;
use serde::{Deserialize, Serialize};
// use std::borrow::Cow;
// use std::fmt::format;

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const BLACK: Color32 = Color32::from_rgb(10, 10, 10);
const CYAN: Color32 = Color32::from_rgb(100, 150, 150);
const RED: Color32 = Color32::from_rgb(180, 0, 0);

#[derive(Serialize, Deserialize)]

pub struct HeadlinesConfig {
    pub dark_mode: bool,
    pub api_key: String,
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

impl Default for HeadlinesConfig {
    fn default() -> Self {
        Self {
            dark_mode: Default::default(),
            api_key: String::new(),
        }
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

        let config: HeadlinesConfig =
            confy::load("headlines", "headlines-config").unwrap_or_default();
        Samachar {
            articles: Vec::from_iter(iter),
            config,
        }
    }

    pub fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for article in &self.articles {
            // Render Title
            ui.add_space(PADDING);
            let title = format!("{}", article.title);

            if self.config.dark_mode {
                ui.colored_label(WHITE, title);
            } else {
                ui.colored_label(BLACK, title);
            }

            // Render Description
            ui.add_space(PADDING);
            let description = Label::new(&article.description);
            ui.add(description);

            // render hyperlinks
            if self.config.dark_mode {
                ui.style_mut().visuals.hyperlink_color = CYAN;
            } else {
                ui.style_mut().visuals.hyperlink_color = RED;
            }

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
                                                                     // let refers_button = ui.add(Button::new("Render"));
                    let theme_button = ui.add(Button::new("Theme"));

                    if theme_button.clicked() {
                        self.config.dark_mode = !self.config.dark_mode;
                    }

                    if close_button.clicked() {
                        frame.close();
                    }
                })
            })
        });
    }

    pub fn render_config(&mut self, ctx: &Context) {
        Window::new("Configuration").show(ctx, |ui| {
            ui.label("Enter you API_KEY for newsapi.org");
            let text_input = ui.text_edit_singleline(&mut self.config.api_key);
            if text_input.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                if let Err(e) = confy::store(
                    "headlines",
                    "headlines",
                    HeadlinesConfig {
                        dark_mode: self.config.dark_mode,
                        api_key: self.config.api_key.to_string(),
                    },
                ) {
                    tracing::error!("Failed to save app state: {}", e)
                };
                tracing::error!("API key set")
            }

            // tracing::error!("{}", &self.config.api_key);
            ui.label("If you don't have the API key, head over to ");
            ui.hyperlink("https://newsapi.org");
        });
    }
}
