// types used by top level server for apis

use serde::{Deserialize,Serialize};
use colored::Colorize;

#[derive(Serialize,Deserialize,Debug)]
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

#[derive(Serialize,Deserialize,Debug)]
pub struct SearchRenameItemsRequest
{
    pub query:String,
    pub simplify:bool
}

#[derive(Serialize,Deserialize,Debug)]
pub struct RenameRequest
{
    pub target:String,
    pub newName:String
}

impl RenameRequest
{
    /** special print the rename request */
    pub fn printRename(&self)
    {
        println!("{}",self.target.yellow());
        println!("{} {}",
            "->".green(),
            self.newName.yellow()
        );
    }
}