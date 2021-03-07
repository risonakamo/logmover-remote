use reqwest::blocking::get;

fn main()->Result<(),Box<dyn std::error::Error>>
{
    let resp=get("http://localhost:4200/log-move")?;
    let resptext=resp.text()?;
    println!("{}",resptext);
    return Ok(());
}