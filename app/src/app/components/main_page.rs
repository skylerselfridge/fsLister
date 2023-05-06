use std::convert::TryInto;

use eframe::egui::Layout;

use super::AppComponent;
use crate::app::App;
use crate::resources::{self, Node};


pub struct MainPage;

impl AppComponent for MainPage {
    type Context = App;

    fn add(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {

        if ctx.fsLister.get_folder_path() != "" {
            let sort_setting = ctx.fsLister.get_sort_setting();
            eframe::egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(ui.available_width());
                });
                
                ui.horizontal(|ui|{
                    let string_path = ctx.fsLister.get_folder_path();
                    let dir = std::path::Path::new(&string_path);
                    let file_struct = ctx.fsLister.get_file_struct();
                    ui.add_space(15.);
                    if file_struct.get_name() == "" {
                        let tree = Node::from_path(dir);
                        ctx.fsLister.set_file_struct(tree);
                    } else {
                        let tree = ctx.fsLister.get_file_struct();
                        pre_order(tree, &dir.to_string_lossy().to_string(), ui, sort_setting);
                    }
                });
                
            
            });
        }
        
        
        
        
    }
}


fn pre_order(root: &Node, dir: &String, ui: &mut eframe::egui::Ui, sort_setting: bool){
    let s;
    if dir.contains(&root.get_name()){
        s = dir.to_owned();
        if resources::is_dir(s.clone()){
            let mut files: Vec<String> = Vec::new();
            resources::get_files(s.clone(), files.as_mut());
          
            let g = format!("{}: {} images",s,files.len());
                        
            eframe::egui::CollapsingHeader::new(g)
                .default_open(true) // set the default state of the header to open
                .id_source(s.clone()) // give the header an ID so we can retrieve its state later
                .show(ui, |ui| {
                    
                    let children = root.get_children();
                    if !sort_setting {
                        for n in children {
                            pre_order(n, &s, ui, sort_setting);
                        }
                    } else {
                        let mut heap = mmheaps::min_heap::min::MinHeap::<&Node>::new();
                        for n in children {
                            //pre_order(n, &s, ui);
                            let child_path = dir.to_owned() + "\\" + &n.get_name();
                            let mut child_files: Vec<String> = Vec::new();
                            resources::get_files(child_path.clone(), child_files.as_mut());
                            let f: i32 = child_files.len().try_into().unwrap();
                            heap.push(n, f);
                        }
                        for n in heap {
                            pre_order(n, &s, ui, sort_setting);
                        }
                    }                            
            });
        }
    } else {
        s = dir.to_owned() + "\\" + &root.get_name();
        if resources::is_dir(s.clone()){
            let mut files: Vec<String> = Vec::new();
            resources::get_files(s.clone(), files.as_mut());
            let substring = format!("{}{}",dir.to_string(),"\\");
            let g = format!("{}: {} images",s.replace(&substring,""),files.len());
            eframe::egui::CollapsingHeader::new(g)
                .default_open(false) // set the default state of the header to open
                .id_source(s.clone()) // give the header an ID so we can retrieve its state later
                .show(ui, |ui| {
                
                        let children = root.get_children();
                        for n in children {
                            pre_order(n, &s, ui,sort_setting);
                        }
                    
            });
        } else {
            ui.horizontal(|ui| {
            ui.with_layout(Layout::left_to_right(), |ui| {
                ui.style_mut().override_text_style = Some(eframe::egui::TextStyle::Monospace);
                ui.scope(|ui|{
                    ui.visuals_mut().override_text_color = Some(eframe::egui::Color32::GREEN);
                    ui.label("->");
                });
                ui.label(s);
            });
        });
        }
    }
    
}