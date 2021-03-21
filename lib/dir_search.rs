use std::fs::{read_dir,ReadDir,DirEntry};
use sublime_fuzzy::{FuzzySearch,Scoring,Match};
use lazy_static::lazy_static;
use std::io;

use super::types::search_types::SearchItem;

/// fuzzy search a target dir for a query. return vector of results
fn searchDir(targetDir:&str,query:&str)->Vec<SearchItem>
{
    let filenames:ReadDir=read_dir(targetDir).unwrap();

    let matchedFiles:Vec<String>=filenames.filter_map(|x:io::Result<DirEntry>|->Option<String> {
        let filename:String=x.unwrap().file_name().to_str().unwrap().to_string();

        if fuzzyMatch(&filename,query)
        {
            return Some(filename);
        }

        return None;
    }).collect();

    println!("{:#?}",matchedFiles);

    return vec![];
}

/// determine if input matches query
fn fuzzyMatch(input:&str,query:&str)->bool
{
    lazy_static!
    {
        static ref scoring:Scoring=Scoring::emphasize_distance();
    }

    let result:Option<Match>=FuzzySearch::new(
        query,
        input
    )
    .score_with(&scoring)
    .case_insensitive()
    .best_match();

    match result
    {
        Some(res)=>{
            return true;
        }

        None=>{
            return false;
        }
    }
}

pub mod test
{
    use super::searchDir;

    pub fn dirsearchtest()
    {
        searchDir(
            r"C:\Users\ktkm\Desktop\videos\vids",
            "Uma Musume"
        );
    }
}