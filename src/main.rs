#![feature(plugin, custom_derive)]
#![feature(fs_read_write)]
#![plugin(rocket_codegen)]

extern crate rand;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;

use std::sync::Mutex;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use rand::{Rng};
use rocket::State;
use rocket::response::NamedFile;
use rocket_contrib::{Json, Template};

mod admin;
mod quiz;

#[derive(Serialize)]
struct DashboardContext {
   teams: [(String, String, isize);6],
   factor: usize,
}

#[get("/")]
fn home(db: State<DbConn>) -> Template {
   let teams = db.lock().expect("show dashboard");
   let context = DashboardContext {
      teams: [("ÉRRE".to_string(), "red".to_string(), teams.r),
               ("GÊ".to_string(), "green".to_string(), teams.g),
               ("BÊ".to_string(), "blue".to_string(), teams.b),
               ("CÊ".to_string(), "cyan".to_string(), teams.c),
               ("ÊME".to_string(), "magenta".to_string(), teams.m),
               ("UAI".to_string(), "yellow".to_string(), teams.y)],
      factor: 4000,
   };
   Template::render("dashboard", &context)
}

#[get("/dashboard")]
fn dashboard(db: State<DbConn>) -> Json {
   let factor = 4000;
   let teams = db.lock().expect("show dashboard");
	Json(json!({
      "points": [teams.r, teams.g, teams.b, teams.c, teams.m, teams.y,],
		"factor": factor,
	}))
}

#[get("/static/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
   NamedFile::open(Path::new("static/").join(file)).ok()
}

struct Teams {
   r: isize,
   g: isize,
   b: isize,
   c: isize,
   m: isize,
   y: isize,
}

type DbConn = Mutex<Teams>;
static NAMES: [&'static str; 6] = ["red", "green", "blue", "cyan", "magenta", "yellow"];

fn main() {
   let mut teams = [0, 0, 0, 0, 0, 0];

   for i in 0..NAMES.len() {
      let mut data = match std::fs::read_string(NAMES[i]) {
         Ok(val) => val,
         Err(_) => {
            std::fs::write(NAMES[i], "0\n").expect(NAMES[i]);
            "0".to_string()
         },
      };
      data.pop();
      teams[i] = match data.parse() {
         Ok(n) => n,
         Err(_) => 0,
      }
   }

   let teams = Teams{
      r: teams[0],
      g: teams[1],
      b: teams[2],
      c: teams[3],
      m: teams[4],
      y: teams[5],
   };
   
   let db = Mutex::new(teams);
	rocket::ignite()
      .manage(db)
      .mount("/", routes![static_files,
                           home,
                           dashboard,
                           admin::get_login,
                           admin::post_login,
                           admin::get_points,
                           admin::post_points,
                           quiz::quiz])
      .attach(Template::fairing())
      .launch();
}
