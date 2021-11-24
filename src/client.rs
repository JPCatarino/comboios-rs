use reqwest;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::Url;
use serde_json::json;

use crate::definitions;
// CP asynchronous client.

pub struct Client{
    req_client: reqwest::Client,
    token: definitions::AccessToken,
}

impl Client{
    const CP_BASE_URL: &'static str = "https://api.cp.pt/cp-api/";

    pub async fn new() -> Self{
        let token;
        let token_url = Url::parse(Self::CP_BASE_URL)
            .unwrap()
            .join("oauth/token")
            .unwrap();
        
        let req_client = reqwest::Client::new();

        let response = req_client
            .post(token_url)
            .header(AUTHORIZATION, "Basic Y3AtbW9iaWxlOnBhc3M=")
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body("grant_type=client_credentials")
            .send()
            .await
            .unwrap();

        
            match response.status() {
                reqwest::StatusCode::OK => {
                    match response.json::<definitions::AccessToken>().await {
                        Ok(parsed) => {token = Some(parsed);}
                        Err(_) => {token = None;}
                    };
                }
                _other => {
                    token = None;
                }
            };
    
        Self{
            token: token.unwrap(),
            req_client: req_client,
        }
    }

    pub async fn get_stations(&self) -> Option<definitions::Stations>{
        let endpoint = "siv/stations";
        
        let response = self.get(endpoint).await;
        let ret_value : Option<definitions::Stations>;

        match response.status() {
            reqwest::StatusCode::OK => {
                match response.json::<definitions::Stations>().await {
                    Ok(parsed) => {
                                    println!("Success! {:?}", parsed);
                                    ret_value = Some(parsed);
                                    
                                  }
                    Err(_) => {ret_value = None;}
                };
            }
            other => {
                panic!("Couldnt fetch info {:?}", other);
            }
        };

        ret_value
    }

    pub async fn get_journeys(&self, origin: &str, destination: &str, date: &str) -> Option<definitions::Journeys>{
        let endpoint = "siv/travel/search";

        let body = json!({
            "departureStationCode": origin,
            "arrivalStationCode": destination,
            "classes": [1, 2],
            "searchType": 3,
            "travelDate": date,
            "returnDate": null,
            "timeLimit": null,
        });

        let response = self.post(endpoint, body).await;
        let ret_value : Option<definitions::Journeys>;

        match response.status() {
            reqwest::StatusCode::OK => {
                match response.json::<definitions::Journeys>().await {
                    Ok(parsed) => {ret_value = Some(parsed);}
                    Err(_) => {ret_value = None;}
                };
            }
            other => {
                panic!("Couldnt fetch info {:?}", other);
            }
        };

        ret_value
    }

    async fn get(&self, endpoint_str: &str) -> reqwest::Response{
        let endpoint_url = Url::parse(Self::CP_BASE_URL)
            .unwrap()
            .join(endpoint_str)
            .unwrap();
        
        let response = self.req_client
            .get(endpoint_url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token.access_token))
            .send()
            .await
            .unwrap();
        
        response
    }

    async fn post(&self, endpoint_str: &str, body: serde_json::Value) -> reqwest::Response{
        let endpoint_url = Url::parse(Self::CP_BASE_URL)
            .unwrap()
            .join(endpoint_str)
            .unwrap();
        
        let response = self.req_client
            .post(endpoint_url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token.access_token))
            .json(&body)
            .send()
            .await
            .unwrap();
        
        response
    }
}