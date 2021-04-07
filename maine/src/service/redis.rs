pub struct Client {
    pub redis: redis::Client,
}

impl Client {
    pub async fn new() -> Self {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        Client { redis: client }
    }
}
