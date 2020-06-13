#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rocket_contrib;

#[cfg(test)] mod tests;

use std::collections::HashMap;

use rocket::Request;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

#[derive(Serialize)]
struct TemplateContext {
    items: Vec<&'static str>
}

#[get("/")]
fn index() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("index", &context)
}

// Components

#[get("/buttons")]
fn buttons() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("buttons", &context)
}

#[get("/cards")]
fn cards() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("cards", &context)
}

// Utilities

#[get("/colors")]
fn colors() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("utilities-color", &context)
}

#[get("/borders")]
fn borders() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("utilities-border", &context)
}

#[get("/animations")]
fn animations() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("utilities-animation", &context)
}

#[get("/others")]
fn others() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("utilities-other", &context)
}

// Pages

#[get("/login")]
fn login() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("login", &context)
}

#[get("/register")]
fn register() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("register", &context)
}

#[get("/reset")]
fn reset() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("forgot-password", &context)
}

#[get("/error")]
fn error() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("404", &context)
}

#[get("/blank")]
fn blank() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("blank", &context)
}

#[get("/charts")]
fn charts() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("charts", &context)
}

#[get("/tables")]
fn tables() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("tables", &context)
}


#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("404", &map)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, buttons, cards, colors, borders, 
                            animations, others, login, register, reset,
                            error, blank, charts, tables])
        .mount("/", StaticFiles::from("templates"))
        .attach(Template::fairing())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}