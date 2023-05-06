use eframe::{egui::Ui, Frame};


mod titlebar;
mod topbar;

mod main_page;


pub use main_page::MainPage;

pub use titlebar::Titlebar;
pub use topbar::Topbar;

#[derive(Debug, PartialEq, Eq)]
pub enum UIPages {
    Main,
}

pub trait AppComponent {
    type Context;

    #[allow(unused)]
    fn add(ctx: &mut Self::Context, ui: &mut Ui) {}

    #[allow(unused)]
    fn with_frame(ctx: &mut Self::Context, ui: &mut Ui, frame: &mut Frame) {}
}
