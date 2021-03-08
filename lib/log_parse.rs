use regex::{Regex,Captures};
use std::error::Error;

use super::types::api_types::MoveItem;

// fn parseFromClipboard()->Result<Vec<MoveItem>,String>
// {

// }

/// parse a log entry line into a move item object.
fn parseLogLine(line:String)->Result<MoveItem,Box<dyn Error>>
{
    let reg:Regex=Regex::new(r"(\d+-\d+-\d+ \d+:\d+:\d+) (.*+)").unwrap();

    // [1]: the date string
    // [2]: the item filename
    let caps:Captures=reg.captures(&line).unwrap();

    if caps.len()!=3
    {
        eprintln!("failed log parse for entry:");
        eprintln!("{}",line);
        return Err("failed parse error")?;
    }

    return Ok(MoveItem {
        name:caps[2].to_string(),
        time:caps[1].to_string()
    });
}

pub mod tests
{
    use super::parseLogLine;

    pub fn logparsetest()
    {
        let testentry:String=concat!(
            "2021-03-07 01:41:59 [Erai-raws] Azur Lane - Bisoku Zenshin! - ",
            "08 [1080p][Multiple Subtitle].mkv"
        ).to_string();

        parseLogLine(testentry).unwrap();
    }
}