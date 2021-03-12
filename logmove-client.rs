#![allow(non_snake_case)]

use reqwest::Client;

use logmover_remote::{
    types::api_types::{LogMoveRequest},
    types::relocation_types::{RelocationResult,printRelocationResult},
    log_parse::parseFromClipboard
};

#[tokio::main]
async fn main()
{
    let logmoveRequest:LogMoveRequest=LogMoveRequest {
        items:parseFromClipboard().unwrap()
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