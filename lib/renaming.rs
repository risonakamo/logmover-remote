use std::path::{Path,PathBuf};
use std::fs::rename;
use colored::Colorize;

/** rename an item in the target dir. returns bool on if successful. */
fn renameItem(targetDir:&str,targetItem:&str,newName:&str)->bool
{
    let pathToFile:PathBuf=Path::new(targetDir).join(targetItem);
    let pathToDestFile:PathBuf=Path::new(targetDir).join(newName);

    match rename(&pathToFile,&pathToDestFile)
    {
        Err(err)=>{
            eprintln!("{}","failed rename of file".red());
            eprintln!("{}",pathToFile.to_str().unwrap().yellow());
            eprintln!("{} {}",
                "->".green(),
                pathToDestFile.to_str().unwrap().yellow()
            );
            eprintln!("{}",err);
            return false;
        }

        Ok(())=>{}
    }

    return true;
}

pub mod test
{
    use super::renameItem;

    pub fn renametest()
    {
        renameItem(
            r"C:\Users\ktkm\Desktop\videos\simulacast\[Nekomoe kissaten&VCB-Studio] Miru Tights [Ma10p_1080p]\SPs",
            r"[Nekomoe kissaten&VCB-Studio] Miru Tights [Menu][Ma10p_1080p][x265_flac].mkv",
            "something.mkv"
        );
    }
}