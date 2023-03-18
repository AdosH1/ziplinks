extern crate redis;
use redis::Commands;
use crate::Settings;
use crate::Link;
use anyhow::Result;
use redis::RedisResult;

fn get_con() -> RedisResult<redis::Connection> {
    let config = Settings::new();
    let settings = config.unwrap();

    let client = redis::Client::open(settings.database.address)?;
    client.get_connection()
}

pub fn get_links(key : String) -> Result<Vec<Link>> {
    let mut con = get_con()?;

    let res : Vec<String> = con.get(key)?;
    let value = res.into_iter().map(|x| Link::try_create(x).unwrap()).collect();

    return Ok(value);
}

pub fn set_links(key : String, value : Vec<Link>) -> redis::RedisResult<()> {
    let mut con = get_con()?;

    let v : Vec<String>  = value.into_iter().map(|x| x.to_string()).collect();
    let _ : () = con.set(key, v)?;

    Ok(())
}


