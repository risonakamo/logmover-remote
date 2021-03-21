use super::types::search_types::SearchItem;

/// fuzzy search a target dir for a query. return vector of results
fn searchDir(targetDir:&str,query:&str)->Vec<SearchItem>
{


    return vec![];
}

pub mod test
{
    use super::searchDir;

    pub fn dirsearchtest()
    {
        searchDir(
            r"C:\Users\ktkm\Desktop\videos\vids",
            "uma"
        );
    }
}