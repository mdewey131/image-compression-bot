#[macro_use]
extern crate rocket;

use rocket::http::{Status, ContentType};
use rocket::form::{Form, Contextual, FromForm, FromFormField, Context};
use rocket::fs::{FileServer, TempFile, relative};

use rocket_dyn_templates::Template;

#[derive(Debug, FromForm)]
#[allow(dead_code)]
struct Submission<'v> {
    #[field(validate = ext(ContentType::MP4).or_else(msg!("{:?}", self.file)))]
    file: TempFile<'v>,
}

#[derive(Debug, FromForm)]
#[allow(dead_code)]
struct Submit<'v> {
    submission: Submission<'v>
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &Context::default())
} 
#[post("/", data = "<form>")]
fn submit<'r>(form: Form<Contextual<'r, Submit<'r>>>) -> (Status, Template) {
    let template = match form.value {
        Some(ref submission) => {
            println!("Submission: {:#?}", submission);
            Template::render("success", &form.context)
        },
        None => {Template::render("index", &form.context)}
    };
    (form.context.status(), template)
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index, submit])
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("/static")));

    Ok(rocket.into())
}