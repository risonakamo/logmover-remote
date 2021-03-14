use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use super::types::configuration_types::{LogMoverConfig,LogMoverClientConfig};

/// retrieve the config yaml from the current dir.
pub fn getConfig()->Result<LogMoverConfig,Box<dyn Error>>
{
    let file:File=File::open("config.yaml")?;
    let reader:BufReader<File>=BufReader::new(file);

    let config:LogMoverConfig=serde_yaml::from_reader(reader)?;

    return Ok(config);
}

/// retrieve the client config yaml
pub fn getConfigClient()->Result<LogMoverClientConfig,Box<dyn Error>>
{
    let file:File=File::open("client-config.yaml")?;
    let reader:BufReader<File>=BufReader::new(file);

    let config:LogMoverClientConfig=serde_yaml::from_reader(reader)?;

    return Ok(config);
}

pub mod tests
{
    use super::getConfig;

    pub fn configtest()
    {
        getConfig().unwrap();
    }
}