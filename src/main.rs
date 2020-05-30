extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::templates::Template;

fn main() {
    rocket::ignite()
        .attach(Template::fairing());
}

#[get("/")]
fn index() -> Template {
    let context = context();
    Template::render("index", &context)
}