// types used for relocation lib

use colored::{Colorize,ColoredString};
use serde::{Serialize,Deserialize};

/// result of a relocation attempt
#[derive(Debug,Serialize,Deserialize)]
pub struct RelocationResult
{
    pub allSuccess:bool, // whether the overall relocation was successful
    pub itemResults:Vec<RelocationItemResult> // individual results of each item
}

/// relocation attempt result for a single item
#[derive(Debug,Serialize,Deserialize)]
pub struct RelocationItemResult
{
    pub name:String,
    pub success:bool
}

/// print relocation result
pub fn printRelocationResult(result:&RelocationResult)
{
    let passString:ColoredString;
    if result.allSuccess
    {
        passString="completed".green();
    }

    else
    {
        passString="aborted".red()
    }

    println!("relocating {} items...{}",
        result.itemResults.len(),
        passString
    );

    for x in &result.itemResults
    {
        printMoveItemResult(x);
    }
}

/// print single item result
fn printMoveItemResult(itemResult:&RelocationItemResult)
{
    let icon:ColoredString;
    let text:ColoredString;

    if itemResult.success
    {
        icon="✓".green();
        text=itemResult.name.yellow();
    }

    else
    {
        icon="✗".red();
        text=itemResult.name.red();
    }

    println!("{} {}",icon,text);
}