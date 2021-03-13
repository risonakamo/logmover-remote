use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct LogMoverConfig
{
    pub target_dir:String,
    pub dest_dir:String
}