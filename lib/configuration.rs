use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use super::types::configuration_types::LogMoverConfig;

fn getConfig()->Result<LogMoverConfig,Box<dyn Error>>
{
    let file:File=File::open("config.yaml")?;
    let reader:BufReader<File>=BufReader::new(file);

    let config:LogMoverConfig=serde_yaml::from_reader(reader)?;

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