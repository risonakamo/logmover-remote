use std::fs::{read_dir,ReadDir,DirEntry};
use sublime_fuzzy::{FuzzySearch,Scoring,Match};
use lazy_static::lazy_static;
use std::io;
use regex::{Regex};
use std::borrow::Cow;
use std::path::Path;

use super::types::search_types::SearchItem;

/// fuzzy search a target dir for a query. return vector of results
pub fn searchDir(targetDir:&str,initialQuery:&str,convertShortname:bool)->Vec<SearchItem>
{
    let query:String;
    if convertShortname
    {
        query=simplifyName(&initialQuery);
    }

    else
    {
        query=initialQuery.to_string();
    }

    // get all the files
    let filenames:ReadDir=read_dir(targetDir).unwrap();

    // perform fuzzy matching on the files, discarding un-matched items
    let matchedFiles:Vec<String>=filenames.filter_map(|x:io::Result<DirEntry>|->Option<String> {
        let filename:String=x.unwrap().file_name().to_str().unwrap().to_string();

        // if file is an excluded file type, skip immediately
        if checkInvalidFileType(&filename)
        {
            return None;
        }

        // the file name to use for matching. either the shortname if shortname enabled
        // or just the filename
        let matchName:String;
        if convertShortname
        {
            matchName=simplifyName(&filename);
        }

        else
        {
            matchName=filename.clone();
        }

        if query.len()==0 || fuzzyMatch(&matchName,&query,-200)
        {
            return Some(filename);
        }

        return None;
    }).collect();

    // convert items to search items and return
    return matchedFiles.iter().map(|x:&String|->SearchItem {
        return SearchItem {
            name:x.clone(),
            shortname:simplifyName(&x)
        };
    }).collect();
}

/// determine if input matches query
fn fuzzyMatch(input:&str,query:&str,scoreThreshold:isize)->bool
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
            if res.score()<scoreThreshold
            {
                return false;
            }

            return true;
        }

        None=>{
            return false;
        }
    }
}

/// perform custom name simplification
fn simplifyName(filename:&str)->String
{
    lazy_static!
    {
        static ref replacer1:Regex=Regex::new(r"[\[\(].*?[\]\)]|\.mkv|\.mp4|END|end").unwrap();
        static ref replacer2:Regex=Regex::new(r"[^\w]|\d").unwrap();
    }

    let afterReplacer1:Cow<str>=replacer1.replace_all(filename,"");
    let res:Cow<str>=replacer2.replace_all(&afterReplacer1,"");

    return res.to_lowercase();
}

/** return if the file has an extension we care about */
fn checkInvalidFileType(filename:&str)->bool
{
    let extension:&str=match Path::new(filename).extension()
    {
        Some(res)=>res.to_str().unwrap(),
        None=>""
    };

    return extension=="ini";
}

pub mod test
{
    use super::searchDir;

    pub fn dirsearchtest()
    {
        let res=searchDir(
            r"C:\Users\ktkm\Desktop\videos\vids",
            "uma musume",
            false
        );

        println!("{:#?}",res);
    }
}