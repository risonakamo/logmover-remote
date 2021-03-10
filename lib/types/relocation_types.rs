/// result of a relocation attempt
#[derive(Debug)]
pub struct RelocationResult
{
    pub allSuccess:bool, // whether the overall relocation was successful
    pub itemResults:Vec<RelocationItemResult> // individual results of each item
}

/// relocation attempt result for a single item
#[derive(Debug)]
pub struct RelocationItemResult
{
    pub name:String,
    pub success:bool
}