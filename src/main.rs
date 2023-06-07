use serde::{Deserialize, Serialize};
use crate::ves::VesClient;

mod mot;
mod ves;

struct Foo {
    curr: u32,
    next: u32
}

impl Foo {
    fn new(start: u32) -> Self {
        Foo { curr: start, next: start * 2 }
    }
}

impl Iterator for Foo {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        if current > 1000 {
            return None;
        }

        self.curr = self.next;
        self.next = self.curr * 2;

        Some(current)
    }
}

#[derive(Deserialize, Serialize)]
struct MyStruct {
    #[serde(rename = "fooBar")]
    foo_bar: String
}

#[tokio::main]
async fn main() {
    let api_key = std::env::var("VES_API_KEY").expect("VES API key must be set");
    let client = VesClient::new(api_key);
    let res = client.get_vehicle("RJ06JOU".to_string()).await.unwrap();
    println!("{}", res);
}

// NOTE: Final goal for this is to have working MOT and VES clients
// - Crawl through all the MOT data pages
// - Fetch VES entry for each vehicle in MOT data
// - Store all info together in database
