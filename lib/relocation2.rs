use std::path::{Path,PathBuf};
use std::fs::rename;
use colored::Colorize;
use lazy_static::lazy_static;
use regex::{Regex};

use super::types::relocation_types::{RelocationResult,RelocationItemResult};

/// attempt to relocate array of files from the specified target directory to the destination directory.
/// aborts move attempt if any one of the provided items is missing. returns result.
pub fn relocateMultiple(targetDir:&str,destinationDir:&str,items:&Vec<String>)->RelocationResult
{
    let existenceResult:RelocationResult=relocationExistence(targetDir,items);

    if !existenceResult.allSuccess
    {
        println!("{}","relocation aborted: unable to find all items".red());
        return existenceResult;
    }

    let mut relocateAllSuccess:bool=true;
    let relocateResult:Vec<RelocationItemResult>=items.into_iter().map(
        |x:&String|->RelocationItemResult {
            let moveSuccess:bool=relocate(targetDir,destinationDir,x);

            if !moveSuccess
            {
                relocateAllSuccess=false;
            }

            return RelocationItemResult {
                name:x.to_string(),
                success:moveSuccess
            };
        }
    ).collect();

    return RelocationResult {
        allSuccess:relocateAllSuccess,
        itemResults:relocateResult
    };
}

/// clean a name so it is just the filename
pub fn cleanName(name:&str)->String
{
    lazy_static!
    {
        static ref replacer:Regex=Regex::new(r"(.*)\.(\S*)").unwrap();
    }

    return replacer.captures(name).unwrap()
        .get(0).unwrap()
        .as_str().to_string();
}

/// given a target dir and array of items, check if each item exists. returns a relocation
/// result where the overall success is true if all items exist. provides result for each
/// item individually.
fn relocationExistence(targetDir:&str,items:&Vec<String>)->RelocationResult
{
    let pathToTarget:&Path=Path::new(targetDir);

    let mut allExist=true;
    let existResults:Vec<RelocationItemResult>=items.into_iter().map(|x:&String|->RelocationItemResult {
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
    use super::{relocationExistence,relocate,relocateMultiple};

    pub fn relocatetest2()
    {
        let result=relocationExistence(
            r"C:\Users\ktkm\Desktop\logmover-remote\testzone\vids",
            &vec![
                "[Erai-raws] World Witches Hasshin Shimasu! - 07 [1080p][Multiple Subtitle].mkv".to_string(),
                // "[Erai-raws] Show by Rock!! Stars!! - 07 [v0][1080p].mkv",
                "[Erai-raws] Azur Lane - Bisoku Zenshin! - 08 [1080p][Multiple Subtitle].mkv".to_string()
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

    pub fn relocatetest4()
    {
        let result=relocateMultiple(
            r"C:\Users\ktkm\Desktop\logmover-remote\testzone\vids",
            r"C:\Users\ktkm\Desktop\logmover-remote\testzone\delete",
            &vec![
                "[Erai-raws] World Witches Hasshin Shimasu! - 07 [1080p][Multiple Subtitle].mkv".to_string(),
                // "[Erai-raws] Show by Rock!! Stars!! - 07 [v0][1080p].mkv",
                "[Erai-raws] Azur Lane - Bisoku Zenshin! - 08 [1080p][Multiple Subtitle].mkv".to_string()
            ]
        );

        println!("{:#?}",result);
    }
}