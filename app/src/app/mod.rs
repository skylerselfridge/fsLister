use std::rc::Rc;

use self::components::UIPages;
use crate::config::{AppConfig};
use crate::{fsLister::fsLister, resources::ResourceLoader};

use eframe::CreationContext;

mod components;
mod egui_app;
mod widgets;

pub type GlWindow = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;

pub struct App {
    window: Rc<GlWindow>,
    resources: ResourceLoader,
    config: AppConfig,
    fsLister: fsLister,
    page: UIPages,
}

impl App {
    pub fn from_config(config: AppConfig, cc: &CreationContext) -> Self {
        let mut resources = ResourceLoader::new(&config);
     

        let fsLister = fsLister::new();
       

        cc.egui_ctx.set_visuals(resources.visuals().clone());
        cc.egui_ctx.set_fonts(resources.fonts());
        resources
            .load_runtime(&config, &cc.egui_ctx)
            .expect("Failed to load Resources::Runtime");

        Self {
            window: cc.win.clone(),

            config,
            resources,

            fsLister,
           

            page: UIPages::Main,
        }
    }

    fn window(&self) -> &glutin::window::Window {
        self.window.window()
    }
}
