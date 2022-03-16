extern crate redis;
use redis::Commands;

struct server {

}

fn fetch_an_integer(redis_client: &mut redis::Connection) -> redis::RedisResult<isize> {
    let _ : () = redis_client.set("my_key", 42)?;
    redis_client.get("some_key")
}

fn main() -> Result<(), dyn std::error::Error> {
    let redis_client = redis::Client::open("redis://127.0.0.1/")?;

    let mut redis_connection = redis_client.get_connection()?;

    println!("Hello, world!");
    let val = fetch_an_integer(&mut redis_connection);
    println!("Got value: {}", val.is_ok());
    println!("Got value: {}", val.unwrap());
}
