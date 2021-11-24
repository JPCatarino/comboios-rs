use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub region: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub designation: String,
    pub code: String,
}

#[derive(Deserialize, Debug)]
#[serde(transparent)] 
pub struct Stations {
    pub stations: Vec<Station>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Journeys {
    pub messages: Vec<String>,
    pub travel_date: String,
    pub return_date: Option<String>,
    pub outward_trip: Vec<OutwardTrip>,
    pub return_trip: Vec<OutwardTrip>,
    pub departure_station: Station,
    pub arrival_station: Station,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutwardTrip {
    pub duration: String,
    pub transfer_count: i64,
    pub saleable_online: bool,
    pub sale_link: Option<String>,
    pub travel_sections: Vec<TravelSection>,
    pub services: String,
    pub arrival_time: String,
    pub departure_time: String,
    pub base_prices: Vec<BasePrice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TravelSection {
    pub duration: String,
    pub train_number: i64,
    pub arrival_time: String,
    pub departure_time: String,
    pub service_code: ServiceCode,
    pub departure_station: Station,
    pub arrival_station: Station,
    pub train_stops: Vec<TrainStop>,
    pub departure_platform: Option<String>,
    pub arrival_platform: Option<String>,
    pub sequence_number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCode {
    pub code: String,
    pub designation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainStop {
    pub platform: Option<String>,
    pub departure: String,
    pub arrival: String,
    pub station: Station,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasePrice {
    pub price_type: i64,
    pub cents_value: i64,
    pub travel_class: i64,
    pub constraints: Vec<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
    pub scope: String,
}
