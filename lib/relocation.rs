use std::{
    fs::rename,
    path::{Path,PathBuf}
};
use colored::Colorize;

use super::types::api_types::MoveItem;

/// perform relocation action for array of move items. return if all of them succeeded
fn relocateMultiple(targetFolder:&str,destFolder:&str,items:&Vec<MoveItem>)->bool
{
    if !relocationPrecheck(targetFolder,items)
    {
        return false;
    }

    return items.into_iter().all(|x:&MoveItem|->bool {
        return relocate(targetFolder,destFolder,&x.name);
    });
}

/// relocate target file given target folder and destination folder. returns
/// bool on success.
fn relocate(targetFolder:&str,destFolder:&str,filename:&str)->bool
{
    let target:PathBuf=Path::new(targetFolder).join(filename);
    let dest:PathBuf=Path::new(destFolder).join(filename);

    let triedTargetStr:Option<&str>=target.to_str();
    let targetStr:String;

    match triedTargetStr
    {
        None=>{
            panic!("path string conversion error");
        }

        Some(x)=>{
            targetStr=String::from(x);
        }
    }

    match rename(target,dest)
    {
        Ok(())=>{
            println!("moved {}",targetStr.yellow());
        }

        Err(err)=>{
            println!("{}","could not locate target:".red());
            println!("{}",targetStr.yellow());
            return false;
        }
    }

    return true;
}

/// given a target path and the items to relocate, check all items for if they
/// exist. if any do not exist, return false. print out any that do not exist.
fn relocationPrecheck(targetDir:&str,items:&Vec<MoveItem>)->bool
{
    let targetPath:&Path=Path::new(targetDir);

    return items.into_iter().all(|x:&MoveItem|->bool {
        let success:bool=targetPath.join(&x.name).exists();

        if !success
        {
            printFailedToLocate(&x.name);
        }

        return success;
    });
}

/// given file name that failed to locate, print error message
fn printFailedToLocate(itemName:&str)
{
    println!("{}","could not locate target:".red());
    println!("{}",itemName.yellow());
}