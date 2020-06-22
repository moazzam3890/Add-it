#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/hi?wave&<num>")]
fn number(num: isize) -> String {
    let result = num +50;

    format!("You've typed {} and the result is {}", num, result)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![number])
    .launch();
}