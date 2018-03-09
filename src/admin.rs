extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;
use rocket::response::Redirect;
use rocket_contrib::{Json, Template};

#[get("/admin")]
fn get_login() -> Template {
   let ctx: HashMap<&str, &str> = HashMap::new();
   Template::render("admin_login", &ctx)
}

#[derive(Deserialize)]
struct LoginData {
   password: String,
}

#[post("/admin", format = "application/json", data = "<data>")]
fn post_login(data: Json<LoginData>) -> Redirect {
   if data.0.password == "adminpassword123" {
      Redirect::to("/")
   } else {
      Redirect::to("/admin")
   }
}
