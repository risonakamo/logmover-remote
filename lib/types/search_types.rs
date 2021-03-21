// types used by fuzzy directory search

/// item found by searchDir
#[derive(Debug)]
pub struct SearchItem
{
    pub name:String,
    pub shortname:String
}