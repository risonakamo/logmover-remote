use std::path::{Path,PathBuf};
use std::fs::rename;
use colored::Colorize;

/** rename an item in the target dir. returns bool on if successful. */
pub fn renameItem(targetDir:&str,targetItem:&str,newName:&str)->bool
{
    let pathToFile:PathBuf=Path::new(targetDir).join(targetItem);
    let pathToDestFile:PathBuf=Path::new(targetDir).join(newName);

    if pathToDestFile.exists()
    {
        printRenameError(&pathToFile,&pathToDestFile);
        eprintln!("{}","destination file already exists".red());
        return false;
    }

    match rename(&pathToFile,&pathToDestFile)
    {
        Err(err)=>{
            printRenameError(&pathToFile,&pathToDestFile);
            eprintln!("{}",err);
            return false;
        }

        Ok(())=>{}
    }

    return true;
}

/** print out a failed rename, given the item and new name */
fn printRenameError(targetItem:&PathBuf,newName:&PathBuf)
{
    eprintln!("{}","failed rename of file".red());
    eprintln!("{}",targetItem.to_str().unwrap().yellow());
    eprintln!("{} {}",
        "->".green(),
        newName.to_str().unwrap().yellow()
    );
}

pub mod test
{
    use super::renameItem;

    pub fn renametest()
    {
        renameItem(
            r"C:\Users\ktkm\Desktop\videos\simulacast\[Nekomoe kissaten&VCB-Studio] Miru Tights [Ma10p_1080p]\SPs",
            r"[Nekomoe kissaten&VCB-Studio] Miru Tights [PV02][Ma10p_1080p][x265_flac].mkv",
            "something2.mkv"
        );
    }
}