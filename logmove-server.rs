#![feature(proc_macro_hygiene,decl_macro)]
#![allow(non_snake_case)]

use rocket::post;
use rocket_contrib::json::{Json,JsonValue};
use rocket_contrib::json;
use colored::Colorize;

use logmover_remote::relocation2::relocateMultiple;

use logmover_remote::types::relocation_types::{RelocationResult,printRelocationResult};
use logmover_remote::types::api_types::{LogMoveRequest,MoveItem};

#[post("/log-move",format="json",data="<request>")]
fn logMove(request:Json<LogMoveRequest>)->JsonValue
{
    let logrequest:LogMoveRequest=request.into_inner();

    println!("=> relocation request for {} items",
        logrequest.items.len().to_string().yellow());

    let moveItems:Vec<String>=logrequest.items.into_iter().map(|x:MoveItem|->String {
        return x.name;
    }).collect();

    let relocateResult:RelocationResult=relocateMultiple(
        r"C:\Users\ktkm\Desktop\logmover-remote\testzone\vids",
        r"C:\Users\ktkm\Desktop\logmover-remote\testzone\delete",
        &moveItems
    );

    printRelocationResult(&relocateResult);

    return json!(relocateResult);
}

fn main()
{
    rocket::ignite().mount("/",rocket::routes![logMove]).launch();
}