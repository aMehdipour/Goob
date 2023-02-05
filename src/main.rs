use std::borrow::Cow;

use eframe::{
    egui::{
        self, CentralPanel, Color32, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label, Layout,
        ScrollArea, Separator, Ui, Vec2,
    },
    epi::App,
    run_native, NativeOptions,
};

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

struct MatlabScript {
    choons: Vec<ChoonsList>,
}

struct ChoonsList {
    title: String,
    url: String,
}

impl MatlabScript {
    fn new() -> Self {
        let iter = (0..20).map(|a: i32| ChoonsList {
            title: format!("Choons List {}", a),
            url: format!("https://choons.io/{}", a),
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
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_choons(ui);
            })
        });
    }

    fn name(&self) -> &str {
        "This is Definitely a MATLAB Script"
    }
}

fn main() {
    let app = MatlabScript::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 960.));
    run_native(Box::new(app), win_option);
}
