#![feature(proc_macro_hygiene,decl_macro)]
#![allow(non_snake_case)]

use rocket::{post,State};
use rocket_contrib::json::{Json,JsonValue};
use rocket_contrib::json;
use rocket_contrib::serve::StaticFiles;
use colored::Colorize;
use yansi::Paint;

use logmover_remote::relocation2::relocateMultiple;
use logmover_remote::logging::logMoveItems;
use logmover_remote::configuration::getConfig;

use logmover_remote::types::relocation_types::{RelocationResult,printRelocationResult};
use logmover_remote::types::api_types::{LogMoveRequest,MoveItem};
use logmover_remote::types::configuration_types::LogMoverConfig;

#[post("/log-move",format="json",data="<request>")]
fn logMove(request:Json<LogMoveRequest>,config:State<LogMoverConfig>)->JsonValue
{
    let logrequest:LogMoveRequest=request.into_inner();

    println!("=> relocation request for {} items",
        logrequest.items.len().to_string().yellow());

    let moveItems:Vec<String>=logrequest.items.iter().map(|x:&MoveItem|->String {
        return x.name.clone();
    }).collect();

    let relocateResult:RelocationResult=relocateMultiple(
        &config.target_dir,
        &config.dest_dir,
        &moveItems
    );

    printRelocationResult(&relocateResult);

    if relocateResult.allSuccess
    {
        logMoveItems(
            &logrequest.items,
            &config.log_path
        );
    }

    return json!(relocateResult);
}

fn main()
{
    Paint::enable_windows_ascii();
    rocket::ignite()
        .manage(getConfig().unwrap())
        .mount("/",rocket::routes![logMove])
        .mount("/remote-rename",StaticFiles::from("remote-rename-web/web"))
        .mount("/build",StaticFiles::from("remote-rename-web/build"))
        .launch();
}