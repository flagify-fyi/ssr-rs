#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::response::content;
use ssr_rs::Ssr;
use std::fs::read_to_string;

#[get("/")]
fn index() -> content::RawHtml<String> {
    let source = read_to_string("./client/dist/ssr/index.js").unwrap();

    let js = Ssr::new(source, "SSR");

    content::RawHtml(js.render_to_string(None))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/styles", FileServer::from("./client/dist/ssr/styles"))
        .mount("/scripts", FileServer::from("./client/dist/client/"))
        .mount("/images", FileServer::from("./client/dist/ssr/images/"))
        .mount("/", routes![index])
}
