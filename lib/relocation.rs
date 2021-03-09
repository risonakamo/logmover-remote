use std::{
    fs::rename,
    path::{Path,PathBuf}
};
use colored::Colorize;

/// relocate target file given target folder and destination folder. returns
/// bool on success.
pub fn relocate(targetFolder:&str,destFolder:&str,filename:&str)->bool
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