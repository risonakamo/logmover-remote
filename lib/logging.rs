use std::fs::{File,remove_file,rename,OpenOptions};
use std::io::{Write,copy};
use std::io;
use tempfile::NamedTempFile;

/// prepend to a specified file the string content.
fn prependFile(filepath:&str,content:&str)->io::Result<()>
{
    let tmpFile:NamedTempFile=NamedTempFile::new()?;
    let tmpPath=tmpFile.into_temp_path();
    let tmpPath=tmpPath.keep()?;

    let mut tmpFile:File=File::create(&tmpPath)?;
    let mut srcFile:File=OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&filepath)?;

    tmpFile.write_all(content.as_bytes())?;
    copy(&mut srcFile,&mut tmpFile)?;

    remove_file(&filepath)?;
    rename(&tmpPath,&filepath)?;
    return Ok(());
}

pub mod tests
{
    use super::prependFile;

    pub fn loggingtest()
    {
        prependFile(
            r"C:\Users\ktkm\Desktop\logmover-remote\testzone\randomise3.log",
            "gekkO\r\n"
        ).unwrap();
    }
}