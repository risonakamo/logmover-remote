use reqwest::Client;

use logmover_remote::types::api_types::{LogMoveRequest,MoveItem};

fn main()
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