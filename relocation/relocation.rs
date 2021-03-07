use std::{
    fs::rename,
    path::{Path,PathBuf}
};

use colored::Colorize;

pub fn relocate(targetFolder:&str,destFolder:&str,filename:&str)->Option<()>
{
    let target:PathBuf=Path::new(targetFolder).join(filename);
    let dest:PathBuf=Path::new(destFolder).join(filename);

    let targetStr:String=String::from(target.to_str()?);

    match rename(target,dest)
    {
        Ok(())=>{

        }

        Err(err)=>{
            println!("{} {}",
                "could not locate target".red(),
                targetStr.yellow()
            );
        }
    }

    return Some(());
}