#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/submit/<num>")]
fn number(num: isize) -> String {
    let result = num +50;

    format!("Your result is {}", result)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![number])
    .launch();
}