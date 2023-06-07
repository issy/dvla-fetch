use std::fmt::Debug;

use chrono::NaiveDate;

pub struct MOTResponse {
    // TODO: Fill in all the fields for the response
}

pub struct MotClient {
    client: reqwest::Client
}

impl MotClient {
    pub fn new(api_key: String) -> MotClient {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-api-key", api_key.parse().unwrap());
        headers.insert("accept", "application/json+v6".parse().unwrap());

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .expect("Failed to build client");

        MotClient{ client }
    }

    fn request_builder(&self) -> reqwest::RequestBuilder {
        self.client
            .get("https://beta.check-mot.service.gov.uk/trade/vehicles/mot-tests")
    }

    async fn get_tests_for_vehicle(&self, registration: String) -> Option<()> {
        self.request_builder()
            .query(&[("registration", registration)])
            .send()
            .await
            .unwrap()
            .json::<()>()
            .await
            .ok()
    }

    pub async fn get_tests_on_date(&self, date: NaiveDate, page_num: i16) -> Option<()> {
        self.request_builder()
            .query(&[("date", date.format("%Y%m%d").to_string()), ("page", page_num.to_string())])
            .send()
            .await
            .unwrap()
            .json::<()>() // TODO: Add type here
            .await
            .ok()
    }

    pub async fn get_vehicles(&self, page_num: i64) -> Option<()> {
        None
    }
}

// TODO: Make an iterator for the client
// - Iterate over pages until max page reached
// - Iterate over pages on a date
