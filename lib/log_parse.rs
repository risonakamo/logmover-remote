use regex::{Regex,Captures};
use std::error::Error;
use lazy_static::lazy_static;
use clipboard_win::get_clipboard_string;
use colored::Colorize;

use super::types::api_types::MoveItem;

/// attempt to pull log entries from the clipboard and return the Move Items
/// from them.
fn parseFromClipboard()->Result<Vec<MoveItem>,Box<dyn Error>>
{
    let clipText:String=get_clipboard_string()
        .unwrap()
        .trim()
        .to_string();

    let splitText=clipText.split("\n")
    .map(|x:&str|->String {
        return x.trim().to_string();
    });

    return Ok(splitText.map(|x:String|->MoveItem {
        return parseLogLine(x).unwrap();
    }).collect());
}

/// parse a log entry line into a move item object.
fn parseLogLine(line:String)->Result<MoveItem,Box<dyn Error>>
{
    lazy_static!
    {
        static ref reg:Regex=Regex::new(r"(\d+-\d+-\d+ \d+:\d+:\d+) (.*+)").unwrap();
    }

    // [1]: the date string
    // [2]: the item filename
    let caps:Captures=match reg.captures(&line) {
        Some(res)=>res,

        None=>{
            return parseLogLineParseErrorPrint(line);
        }
    };

    if caps.len()!=3
    {
        return parseLogLineParseErrorPrint(line);
    }

    return Ok(MoveItem {
        name:caps[2].to_string(),
        time:caps[1].to_string()
    });
}

/// print error for parse log line
fn parseLogLineParseErrorPrint<T>(line:String)->Result<T,Box<dyn Error>>
{
    eprintln!("{}","failed log parse for entry:".red());
    eprintln!("{}",line.yellow());
    return Err("failed parse error")?;
}

pub mod tests
{
    use super::{parseLogLine,parseFromClipboard};
    use super::super::types::api_types::MoveItem;

    pub fn logparsetest()
    {
        let testlog:String=r"
            2021-03-07 01:41:59 [Erai-raws] Azur Lane - Bisoku Zenshin! - 08 [1080p][Multiple Subtitle].mkv
            2021-03-07 01:27:36 [Erai-raws] World Witches Hasshin Shimasu! - 07 [1080p][Multiple Subtitle].mkv
        ".trim().to_string();

        let splitlog:Vec<String>=testlog.split("\n").map(|x:&str|->String {
            return x.trim().to_string();
        }).collect();

        println!("{:#?}",splitlog);

        let moveitems:Vec<MoveItem>=splitlog.into_iter()
        .map(|x:String|->MoveItem {
            return parseLogLine(x).unwrap();
        }).collect();

        println!("{:#?}",moveitems);
    }

    pub fn logparsetest2()
    {
        println!("{:#?}",parseFromClipboard().unwrap());
    }
}