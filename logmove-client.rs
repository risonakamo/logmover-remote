#![allow(non_snake_case)]

use reqwest::Client;

use logmover_remote::types::api_types::{LogMoveRequest,MoveItem};

#[tokio::main]
async fn main()
{
    let testrequest:LogMoveRequest=LogMoveRequest {
        items:vec![
            MoveItem {
                name:"something".to_string(),
                time:"12asdasfs".to_string()
            }
        ]
    };

    let client=Client::new();
    let res:String=client.post("http://localhost:4200/log-move")
        .json(&testrequest)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}",res);
}

#[test]
fn serialisationTest()
{
    let testrequest:LogMoveRequest=LogMoveRequest {
        items:vec![
            MoveItem {
                name:"something".to_string(),
                time:"12asdasfs".to_string()
            }
        ]
    };

    println!("{:#?}",&testrequest);

    let serialised:String=serde_json::to_string(&testrequest).unwrap();
    println!("{}",serialised);

    let deserialised:LogMoveRequest=serde_json::from_str(&serialised).unwrap();

    println!("{:#?}",&deserialised);
}