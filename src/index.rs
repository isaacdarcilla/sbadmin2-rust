use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    let context = context();
    Template::render("index", &context)
}