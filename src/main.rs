mod controllers;
mod models;
mod views;

use gio::prelude::*;
use gio::Resource;
use models::constants;
use std::env::var;
use std::env::args;
use views::ui;

/// Finds where the resources are located and loads them
/// This first checks the local folder, then all XDG dirs
/// Finally it tries flatpak paths
fn load_resources() -> Resource {
    const RES_NAME: &str = "com.gigitux.youp.gresource";
    const APP_NAME: &str = "com.gigitux.youp";

    let mut data_dirs = vec![String::from("data/")];

    if let Ok(var) = var("XDG_DATA_DIRS") {
        for path in var.split(":") {
            data_dirs.push(format!("{}/{}", path, APP_NAME));
        }
    }

    data_dirs.push(format!("/app/share/{}", APP_NAME));

    for path in data_dirs.drain(..) {
        if let Ok(resource) = Resource::load(format!("{}/{}", path, RES_NAME)) {
            return resource;
        }
    }

    panic!("Failed to find adata files!");
}

fn main() {
    let application = gtk::Application::new(Some(constants::APPLICATION_NAME), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|application| application.hold());

    let res = load_resources();
    gio::resources_register(&res);

    application.connect_activate(|app| {
        ui::build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
