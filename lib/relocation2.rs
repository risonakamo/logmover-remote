use super::types::relocation_types::RelocationResult;

/// attempt to relocate array of files from the specified target directory to the destination directory.
/// aborts move attempt if any one of the provided items is missing. returns result.
fn relocateMultiple(targetDir:&str,destinationDir:&str,items:&Vec<&str>)->RelocationResult
{

}

/// given a target dir and array of items, check if each item exists. returns a relocation
/// result where the overall success is true if all items exist. provides result for each
/// item individually.
fn relocationExistence(targetDir:&str,items:&Vec<&str>)->RelocationResult
{

}

pub mod tests
{
    pub fn relocatetest2()
    {

    }
}