#![feature(proc_macro_hygiene,decl_macro)]
#![allow(non_snake_case)]

use rocket::{post,State,get};
use rocket::response::Redirect;
use rocket_contrib::json::{Json,JsonValue};
use rocket_contrib::json;
use rocket_contrib::serve::StaticFiles;
use colored::Colorize;
use yansi::Paint;

use logmover_remote::relocation2::relocateMultiple;
use logmover_remote::logging::logMoveItems;
use logmover_remote::configuration::getConfig;
use logmover_remote::dir_search::searchDir;
use logmover_remote::renaming::renameItem;

use logmover_remote::types::relocation_types::{RelocationResult,printRelocationResult};
use logmover_remote::types::api_types::{LogMoveRequest,MoveItem,SearchRenameItemsRequest,RenameRequest};
use logmover_remote::types::configuration_types::LogMoverConfig;

/// request log move api. body is log move request. returns the status of all items attempted to move
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

/// search for renameable items. post request where body is plain text string to search for.
/// returns array of RenameItems matching the search
#[post("/search-rename-items",format="json",data="<request>")]
fn searchRenameItems(request:Json<SearchRenameItemsRequest>,config:State<LogMoverConfig>)->JsonValue
{
    let searchRequest:SearchRenameItemsRequest=request.into_inner();

    return json!(searchDir(
        &config.target_dir,
        &searchRequest.query,
        searchRequest.simplify
    ));
}

#[post("/rename-item",format="json",data="<request>")]
fn renameItemApi(request:Json<RenameRequest>,config:State<LogMoverConfig>)->JsonValue
{
    let renameRequest:RenameRequest=request.into_inner();

    println!("rename request");
    println!("{:#?}",renameRequest);

    return json!(renameItem(
        &config.target_dir,
        &renameRequest.target,
        &renameRequest.newName
    ));
}

#[get("/")]
fn renameRenameRedirect()->Redirect
{
    return Redirect::to("/remote-rename");
}

fn main()
{
    Paint::enable_windows_ascii();
    rocket::ignite()
        .manage(getConfig().unwrap())
        .mount("/",rocket::routes![logMove,searchRenameItems,renameItemApi,renameRenameRedirect])
        .mount("/remote-rename",StaticFiles::from("remote-rename-web/web"))
        .mount("/build",StaticFiles::from("remote-rename-web/build"))
        .mount("/assets",StaticFiles::from("remote-rename-web/assets"))
        .launch();
}