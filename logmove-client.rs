#![allow(non_snake_case)]
#![allow(unused_variables)]

use reqwest::Client;

use logmover_remote::{
    types::api_types::{LogMoveRequest},
    types::relocation_types::{RelocationResult,printRelocationResult},
    log_parse::parseFromClipboard
};

#[tokio::main]
async fn main()
{
    let items=match parseFromClipboard() {
        Err(err)=>std::process::exit(0),
        Ok(res)=>res
    };

    let logmoveRequest:LogMoveRequest=LogMoveRequest {
        items
    };

    let client=Client::new();
    let res:String=client.post("http://localhost:4200/log-move")
        .json(&logmoveRequest)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let relocationResult:RelocationResult=serde_json::from_str(&res).unwrap();

    printRelocationResult(&relocationResult);
}