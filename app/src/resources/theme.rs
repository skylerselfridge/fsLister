use crate::config::AppStyle;
use eframe::egui::{Visuals};

pub struct Theme {
    visuals: Visuals,
}

impl Theme {
    pub fn new(cfg: &AppStyle) -> Self {
        let mut visuals = Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = cfg.background;
        visuals.widgets.noninteractive.fg_stroke.color = cfg.foreground;
        visuals.widgets.noninteractive.rounding = 0.0.into();

        Self {
            visuals,
        }
    }

    pub fn visuals(&self) -> &Visuals {
        &self.visuals
    }

}
