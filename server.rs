#![feature(proc_macro_hygiene,decl_macro)]
#![allow(non_snake_case)]

use rocket::post;
use rocket_contrib::json::{Json};

use logmover_remote::types::api_types::LogMoveRequest;

#[post("/log-move",format="json",data="<request>")]
fn logMove(request:Json<LogMoveRequest>)->&'static str
{
    println!("did log move");
    println!("{:?}",request);
    return "hey";
}

fn main()
{
    rocket::ignite().mount("/",rocket::routes![logMove]).launch();
}