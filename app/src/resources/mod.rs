use self::{runtime::Runtime, theme::Theme};
use crate::config::AppConfig;
use anyhow::Result;
use eframe::egui::{Context, FontDefinitions, Visuals};
mod file_system;

pub use file_system::Node;
pub use file_system::get_files;
pub use file_system::is_dir;

pub mod icon;

mod fonts;
mod icon_loader;
mod runtime;
mod theme;

pub use self::icon_loader::Icons;

pub struct ResourceLoader {
    theme: Theme,
    runtime: Option<Runtime>,
}

impl ResourceLoader {
    pub fn new(config: &AppConfig) -> Self {
        // TODO: Show error
       

        ResourceLoader {
            theme: Theme::new(&config.style),
            runtime: None,
        }
    }

    fn runtime(&self) -> &Runtime {
        // TODO: Unwrap unchecked?
        self.runtime
            .as_ref()
            .expect("Resources runtime not allocated")
    }

    pub fn load_runtime(&mut self, cfg: &AppConfig, ctx: &Context) -> Result<()> {
        let runtime = Runtime::new(cfg, ctx)?;
        self.runtime = Some(runtime);
        Ok(())
    }

    pub fn fonts(&self) -> FontDefinitions {
        fonts::load_fonts()
    }

    pub fn visuals(&self) -> &Visuals {
        self.theme.visuals()
    }

    pub fn icons(&self) -> &Icons {
        &self.runtime().icons
    }

}
