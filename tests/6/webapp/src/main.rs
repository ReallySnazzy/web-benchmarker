use ohkami::prelude::*;

async fn hello() -> String {
    "Hello, world!".to_owned()
}

#[tokio::main]
async fn main() {
    Ohkami::new(("/hello".GET(hello))).howl("localhost:3000").await
}
