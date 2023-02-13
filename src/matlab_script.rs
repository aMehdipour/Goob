use std::borrow::Cow;

use eframe::{
    egui::{
        self, Button, CentralPanel, Color32, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label,
        Layout, ScrollArea, Separator, TopBottomPanel, Ui, Vec2,
    },
    epi::App,
    run_native, NativeOptions,
};

pub const PADDING: f32 = 5.0;
pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
pub const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct MatlabScript {
    choons: Vec<ChoonsList>,
}

pub struct ChoonsList {
    title: String,
    url: String,
}

impl MatlabScript {
    pub fn new(titles: Vec<String>, urls: Vec<String>) -> Self {
        let iter = titles
            .iter()
            .zip(urls.iter())
            .map(|(title, url)| ChoonsList {
                title: title.to_string(),
                url: url.to_string(),
            });

        MatlabScript {
            choons: Vec::from_iter(iter),
        }
    }

    fn configure_fonts(&self, ctx: &CtxRef) {
        let mut font_def = FontDefinitions::default();

        font_def.font_data.insert(
            "FiraCode".to_string(),
            Cow::Borrowed(include_bytes!(
                "../Fira Code Regular Nerd Font Complete.ttf"
            )),
        );

        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (FontFamily::Proportional, 35.),
        );

        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body,
            (FontFamily::Proportional, 20.),
        );

        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "FiraCode".to_string());

        ctx.set_fonts(font_def);
    }

    fn render_choons(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.choons {
            // let label = Label::new(&a.url).text_style(eframe::egui::TextStyle::Button);
            let title = format!("▶ {}", a.title);

            ui.add_space(PADDING);
            ui.colored_label(WHITE, title);
            ui.add_space(PADDING);
            // ui.add(label);
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::left_to_right(), |ui| {
                ui.add(Hyperlink::new(&a.url).text("click me daddy ⤴"));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub(crate) fn render_top_panel(&self, ctx: &CtxRef, frame: &mut eframe::epi::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new("🍓🍎🍓").text_style(egui::TextStyle::Heading))
                });
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let close_button = ui.add(Button::new("🔫").text_style(egui::TextStyle::Body));
                    if close_button.clicked() {
                        frame.quit();
                    }
                    // let refresh_button =
                    //     ui.add(Button::new("🔄").text_style(egui::TextStyle::Body));
                })
            });
            ui.add_space(10.);
        });
    }
}

impl App for MatlabScript {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx)
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame) {
        self.render_top_panel(ctx, frame);
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_choons(ui);
            });
            render_footer(ctx);
        });
    }

    fn name(&self) -> &str {
        "This is Definitely a MATLAB Script"
    }
}

pub fn render_footer(ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            // ui.add_space(10.);
            ui.add(Label::new("Bottom Text".to_string()));
            ui.add_space(10.);
            ui.add(
                Hyperlink::new(
                    "https://www.youtube.com/playlist?list=PLbo9hNXX38Ytwm5oe6S9S8SBrDTNwvoHF",
                )
                .text("View the Full Garden"),
            );
            ui.add_space(10.);
        })
    });
}

pub fn render_header(ui: &mut egui::Ui) {
    let sep = Separator::default().spacing(20.);

    ui.vertical_centered(|ui| {
        ui.heading("This is Most Definitely a MATLAB Script");
    });
    ui.add_space(PADDING);
    ui.add(sep);
}
