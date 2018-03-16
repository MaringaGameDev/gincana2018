use ::std::collections::HashMap;
use ::rocket::State;
use ::rocket::http::{Cookie, Cookies};
use ::rocket::response::Redirect;
use ::rocket::request::Form;
use ::rocket_contrib::Template;

extern crate bcrypt;
use self::bcrypt::verify;

const PASSWORD: &'static str = "$2y$12$HdiZ1FybVjyzFqOElZneVuDIHykYgTCSWxoBjMtYgx1BrrzQoAwJq";

#[get("/admin")]
fn get_login() -> Template {
   let ctx: HashMap<&str, &str> = HashMap::new();
   Template::render("admin_login", &ctx)
}

#[derive(FromForm)]
struct LoginData {
   password: String,
}

#[post("/admin", data = "<data>")]
fn post_login(data: Form<LoginData>, mut cookies: Cookies) -> Redirect {
   match verify(&data.get().password, PASSWORD) {
      Ok(_)  => {
         cookies.add_private(Cookie::new("auth", "true"));
         Redirect::to("/")
      },
      Err(_) => Redirect::to("/admin"),
   }
}

#[get("/points")]
fn get_points() -> Template {
   let ctx: HashMap<&str, &str> = HashMap::new();
   Template::render("admin_points", &ctx)
}

#[derive(FromForm)]
struct PointsData {
   team: String,
   value: isize,
}

#[post("/points", data = "<data>")]
fn post_points(db: State<::DbConn>, data: Form<PointsData>, mut cookies: Cookies) -> Redirect {
   match cookies.get_private("auth") {
      Some(cookie) => {
         match cookie.value() {
            "true" => {
               let value = &data.get().value;
               if let Err(_) = match &data.get().team as &str {
                  "r" => {
                     db.lock().expect("lock db").r += value;
                     ::std::fs::write("red",
                        db.lock().expect("lock db").r.to_string() + &"\n").expect("red");
                     Ok(())
                  },
                  "g" => {
                     db.lock().expect("lock db").g += value;
                     ::std::fs::write("green",
                        db.lock().expect("lock db").g.to_string() + &"\n").expect("green");
                     Ok(())
                  },
                  "b" => {
                     db.lock().expect("lock db").b += value;
                     ::std::fs::write("blue",
                        db.lock().expect("lock db").b.to_string() + &"\n").expect("blue");
                     Ok(())
                  },
                  "c" => {
                     db.lock().expect("lock db").c += value;
                     ::std::fs::write("cyan",
                        db.lock().expect("lock db").c.to_string() + &"\n").expect("cyan");
                     Ok(())
                  },
                  "m" => {
                     db.lock().expect("lock db").m += value;
                     ::std::fs::write("magenta",
                        db.lock().expect("lock db").m.to_string() + &"\n").expect("magenta");
                     Ok(())
                  },
                  "y" => {
                     db.lock().expect("lock db").y += value;
                     ::std::fs::write("yellow",
                        db.lock().expect("lock db").y.to_string() + &"\n").expect("yellow");
                     Ok(())
                  },
                  _ => Err(()),
               } {
                  Redirect::to("/error1")
               } else {
                  Redirect::to("/points")
               }
            },
            _ => Redirect::to("/error2"),
         }
      },
      None => Redirect::to("/error3"),
   }
}
