use super::{AppComponent};
use crate::{app::{widgets::IconButton, App}, resources::{Node}};
use eframe::egui::Layout;

pub struct Topbar;

impl AppComponent for Topbar {
    type Context = App;

    fn add(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add_space(15.0);

                let settings = IconButton::new(&ctx.resources.icons().folder_open);
                if ui.add(settings).clicked() {
                    let result = nfd::open_pick_folder(None);
                    match result {
                        Ok(nfd::Response::Okay(folder_path)) => {
                            println!("Result: {:?}", folder_path);
                            ctx.fsLister.set_folder_path(folder_path);
                            ctx.fsLister.set_file_struct(Node::new());
                        },
                        Ok(nfd::Response::Cancel) => {
                            println!("User canceled the dialog");
                        },
                        Ok(nfd::Response::OkayMultiple(_)) => todo!(),
                        Err(error) => {
                            println!("Error: {:?}", error);
                        },
                    }
                }
                let sort_setting = ctx.fsLister.get_sort_setting();
                if !sort_setting {
                    let sort_button = IconButton::new(&ctx.resources.icons().sort_on);
                    if ui.add(sort_button).clicked() {
                        ctx.fsLister.set_sort_setting(true);
                    }
                } else {
                    let sort_button = IconButton::new(&ctx.resources.icons().sort_off);
                    if ui.add(sort_button).clicked() {
                        ctx.fsLister.set_sort_setting(false);
                    }
                }
                
            });
        });
    }
}
