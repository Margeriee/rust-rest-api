#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let basci_string = "Hello!";

    rocket::ignite().mount("/", routes![index]).launch();
}