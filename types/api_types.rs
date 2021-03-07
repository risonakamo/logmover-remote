use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveItem
{
    pub name:String,
    pub time:String
}

#[derive(Serialize,Deserialize,Debug)]
pub struct LogMoveRequest
{
    pub items:Vec<MoveItem>
}