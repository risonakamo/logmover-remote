#![feature(proc_macro_hygiene,decl_macro)]
#![allow(non_snake_case)]

use rocket::get;

#[get("/log-move")]
fn logMove()->&'static str
{
    println!("did log move");
    return "hey";
}

fn main()
{
    rocket::ignite().mount("/",rocket::routes![logMove]).launch();
}