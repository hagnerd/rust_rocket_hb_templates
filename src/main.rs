#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct Index {
    name: String,
}

#[get("/")]
fn index() -> Template {
    let context = Index { name: String::from("Matt") };
    Template::render("index", context)
}

#[derive(Serialize)]
struct About {
    topic: String,
}

#[get("/about/<topic>")]
fn about(topic: String) -> Template {
    let context = About { topic };
    Template::render("about", context)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index]).launch();

}
