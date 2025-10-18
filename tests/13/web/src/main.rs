#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment()
            .merge(("address", "0.0.0.0"))
            .merge(("port", 3000)))
        .mount("/", routes![index])
}
