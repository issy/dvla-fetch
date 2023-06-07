use std::error::Error;
use chrono::NaiveDate;
use reqwest::header::HeaderMap;
use reqwest::Response;
use serde::{Serialize, Deserialize, Deserializer};

pub struct VesClient {
    client: reqwest::Client
}

#[derive(Serialize)]
struct RequestBody {
    #[serde(rename = "registrationNumber")]
    registration_number: String
}

#[derive(Deserialize, Debug)]
pub enum VehicleTaxStatus {
    Taxed,
    UnTaxed,
    #[serde(rename = "Not Taxed for on Road Use")]
    NotTaxedForOnRoadUse,
    SORN
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VehicleResponse {
    #[serde(rename = "registrationNumber")]
    registration: String,
    tax_status: VehicleTaxStatus
}

impl VesClient {
    pub fn new(api_key: String) -> VesClient {
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", api_key.parse().unwrap());
        headers.insert("content-type", "application/json".parse().unwrap());

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .expect("Failed to build client");
        VesClient{ client }
    }

    pub async fn get_vehicle(&self, registration_number: String) -> Option<String> {
        let res = self.client
            .post("https://driver-vehicle-licensing.api.gov.uk/vehicle-enquiry/v1/vehicles")
            .body(serde_json::to_string(&RequestBody{ registration_number }).unwrap())
            .send()
            .await
            .ok()
            .filter(|r| { r.status().is_success() });

        return if res.is_some() {
            res.unwrap()
                .text()
                .await
                .ok()
        } else {
            None
        }
    }
}
