#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rand;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::path::{Path, PathBuf};
use rand::{Rng};
use rocket::response::NamedFile;
use rocket_contrib::{Json, Template};

mod admin;

#[derive(Serialize)]
struct DashboardContext {
   teams: [(String, String, u32);6],
   factor: u32,
}

#[get("/")]
fn home() -> Template {
   let context = DashboardContext {
      teams: [("ÉRRE".to_string(), "red".to_string(), 1000),
               ("GÊ".to_string(), "green".to_string(), 2000),
               ("BÊ".to_string(), "blue".to_string(), 3000),
               ("CÊ".to_string(), "cyan".to_string(), 4000),
               ("ÊME".to_string(), "magenta".to_string(), 5000),
               ("UAI".to_string(), "yellow".to_string(), 6000)],
      factor: 20000,
   };
   Template::render("dashboard", &context)
}

#[get("/dashboard")]
fn dashboard() -> Json {
   let factor = 20000;
	Json(json!({
		"points": [rand::thread_rng().gen_range(1000, factor),
                  rand::thread_rng().gen_range(1000, factor),
                  rand::thread_rng().gen_range(1000, factor),
                  rand::thread_rng().gen_range(1000, factor),
                  rand::thread_rng().gen_range(1000, factor),
                  rand::thread_rng().gen_range(1000, factor)],
		"factor": factor,
	}))
}

#[get("/static/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
   NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
	rocket::ignite()
      .mount("/", routes![static_files,
                           home,
                           dashboard,
                           admin::get_login,
                           admin::post_login])
      .attach(Template::fairing())
      .launch();
}
