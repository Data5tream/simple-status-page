use redis::Connection;

pub fn get_redis_connection() -> Connection {
    redis::Client::open("redis://127.0.01/")
        .expect("unable create redis client")
        .get_connection()
        .expect("unable to get redis connection")
}
