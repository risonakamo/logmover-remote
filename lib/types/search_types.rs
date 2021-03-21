// types used by fuzzy directory search

use serde::Serialize;

/// item found by searchDir
#[derive(Debug,Serialize)]
pub struct SearchItem
{
    pub name:String,
    pub shortname:String
}