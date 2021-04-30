#![allow(non_snake_case)]
#![allow(unused_variables)]

use reqwest::Client;
use yansi::Paint;
use std::process::exit;

use logmover_remote::{
    types::api_types::{LogMoveRequest,MoveItem},
    types::relocation_types::{RelocationResult,printRelocationResult},
    types::configuration_types::LogMoverClientConfig,
    log_parse::parseFromClipboard,
    configuration::getConfigClient
};

#[tokio::main]
async fn main()
{
    Paint::enable_windows_ascii();

    let config:LogMoverClientConfig=match getConfigClient() {
        Ok(res)=>res,
        Err(err)=>{
            eprintln!("{}",Paint::red("get config error"));
            eprintln!("{}",err);
            exit(0);
        }
    };

    let items:Vec<MoveItem>=match parseFromClipboard() {
        Err(err)=>{
            eprintln!("{}",Paint::red("clipboard parse error"));
            exit(0);
        },
        Ok(res)=>res
    };

    let logmoveRequest:LogMoveRequest=LogMoveRequest {
        items
    };

    let client=Client::new();
    let res:String=client.post(&format!("{}/log-move",constructConnectionUrl(&config)))
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

/// format client config into server url
fn constructConnectionUrl(config:&LogMoverClientConfig)->String
{
    return format!("http://{}:{}",config.hostname,config.port);
}