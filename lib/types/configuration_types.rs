use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct LogMoverConfig
{
    pub target_dir:String,
    pub dest_dir:String,

    pub log_path:String
}

#[derive(Debug,Deserialize)]
pub struct LogMoverClientConfig
{
    pub hostname:String,
    pub port:String
}