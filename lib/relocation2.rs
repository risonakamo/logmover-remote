use std::path::Path;
use colored::Colorize;

use super::types::relocation_types::{RelocationResult,RelocationItemResult};

/// attempt to relocate array of files from the specified target directory to the destination directory.
/// aborts move attempt if any one of the provided items is missing. returns result.
fn relocateMultiple(targetDir:&str,destinationDir:&str,items:&Vec<&str>)->RelocationResult
{
    let existenceResult:RelocationResult=relocationExistence(targetDir,items);

    if !existenceResult.allSuccess
    {
        println!("{}","relocation aborted: unable to find all items".red());
        return existenceResult;
    }

    return existenceResult;
}

/// given a target dir and array of items, check if each item exists. returns a relocation
/// result where the overall success is true if all items exist. provides result for each
/// item individually.
fn relocationExistence(targetDir:&str,items:&Vec<&str>)->RelocationResult
{
    let pathToTarget:&Path=Path::new(targetDir);

    let mut allExist=true;
    let existResults:Vec<RelocationItemResult>=items.into_iter().map(|x:&&str|->RelocationItemResult {
        let exists:bool=pathToTarget.join(x).exists();

        if !exists
        {
            allExist=false;
        }

        return RelocationItemResult {
            name:x.to_string(),
            success:exists
        };
    }).collect();

    return RelocationResult {
        allSuccess:allExist,
        itemResults:existResults
    };
}

pub mod tests
{
    use super::relocationExistence;

    pub fn relocatetest2()
    {
        let result=relocationExistence(
            r"C:\Users\ktkm\Desktop\logmover-remote\testzone\vids",
            &vec![
                "[Erai-raws] World Witches Hasshin Shimasu! - 07 [1080p][Multiple Subtitle].mkv",
                // "[Erai-raws] Show by Rock!! Stars!! - 07 [v0][1080p].mkv",
                "[Erai-raws] Azur Lane - Bisoku Zenshin! - 08 [1080p][Multiple Subtitle].mkv"
            ]
        );

        println!("{:#?}",result);
    }
}