use super::types::api_types::MoveItem;

// fn parseFromClipboard()->Result<Vec<MoveItem>,String>
// {

// }

fn parseLogLine(line:String)->Option<MoveItem>
{

    return None;
}

pub mod tests
{
    pub fn logparsetest()
    {
        let testentry:String=concat!("2021-03-07 01:41:59 [Erai-raws] Azur Lane - Bisoku Zenshin! - ",
            "08 [1080p][Multiple Subtitle].mkv").to_string();

        println!("{}",testentry);
    }
}