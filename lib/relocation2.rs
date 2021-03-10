use std::path::{Path,PathBuf};
use std::fs::rename;
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

/// relocate a file in the target directory to a destination directory.
/// return bool on success.
fn relocate(targetDir:&str,destinationDir:&str,filename:&str)->bool
{
    let pathToFile:PathBuf=Path::new(targetDir).join(filename);
    let pathToDestFile:PathBuf=Path::new(destinationDir).join(filename);

    match rename(&pathToFile,&pathToDestFile)
    {
        Ok(())=>{}

        Err(err)=>{
            eprintln!("{}","error relocating file:".red());
            eprintln!("{}",pathToFile.to_str().unwrap().yellow());
            eprintln!("{}",err);
            return false;
        }
    }

    return true;
}

pub mod tests
{
    use super::{relocationExistence,relocate};

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

    pub fn relocatetest3()
    {
        let result=relocate(
            r"C:\Users\ktkm\Desktop\logmover-remote\testzone\vids",
            r"C:\Users\ktkm\Desktop\logmover-remote\testzone\delete",
            r"[Erai-raws] World Witches Hasshin Shimasu! - 07 [1080p][Multiple Subtitle].mkv"
        );

        println!("{}",result);
    }
}