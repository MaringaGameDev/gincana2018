#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_cors;

#[macro_use] extern crate rocket_contrib;

use rocket::http::Method;
use rocket_contrib::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};


#[get("/")]
fn dashboard() {}

#[get("/dashboard")]
fn dash_info() -> Json {
	Json(json!({
		"data": [1000, 1000, 1000, 1000, 1000, 1000],
		"factor": 20000
	}))
	
}

#[get("/admin")]
fn admin_login() {}

fn cors_options() -> Cors {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["*"]);

    // You can also deserialize this
    rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
}

fn main() {
	rocket::ignite().mount("/", routes![dashboard, dash_info]).mount("/", rocket_cors::catch_all_options_routes()).manage(cors_options()).launch();
}
