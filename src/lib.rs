use std::collections::HashMap;

pub struct ApiClient {
    user_id: String,
    license_key: String,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new_client(user_id: String, license_key: String) -> ApiClient {
        let client = reqwest::Client::new();
        ApiClient {
            user_id,
            license_key,
            client,
        }
    }

    async fn make_request(
        &self,
        prefix: String,
        ip_address: String,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}{}", prefix, ip_address);

        let resp = self
            .client
            .get(url)
            .basic_auth(&self.user_id, Some(&self.license_key))
            .send()
            .await?
            .json::<Response>()
            .await?;
        Ok(resp)
    }

    pub async fn get_country(
        &self,
        ip_address: String,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        self.make_request(
            String::from("https://geoip.maxmind.com/geoip/v2.1/country/"),
            ip_address,
        )
        .await
    }

    pub async fn get_city(
        &self,
        ip_address: String,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        self.make_request(
            String::from("https://geoip.maxmind.com/geoip/v2.1/city/"),
            ip_address,
        )
        .await
    }

    pub async fn get_insights(
        &self,
        ip_address: String,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        self.make_request(
            String::from("https://geoip.maxmind.com/geoip/v2.1/insights/"),
            ip_address,
        )
        .await
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct City {
    pub confidence: Option<i64>,
    pub geoname_id: Option<i64>,
    pub names: Option<HashMap<String, String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Continent {
    pub code: Option<String>,
    pub geoname_id: Option<i64>,
    pub names: Option<HashMap<String, String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Country {
    pub confidence: Option<i64>,
    pub geoname_id: Option<i64>,
    pub is_in_european_union: Option<bool>,
    pub iso_code: Option<String>,
    pub names: Option<HashMap<String, String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Location {
    pub accuracy_radius: Option<i64>,
    pub average_income: Option<i64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub metro_code: Option<i64>,
    pub population_density: Option<i64>,
    pub time_zone: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Postal {
    pub code: Option<String>,
    pub confidence: Option<i64>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RegisteredCountry {
    pub geoname_id: Option<i64>,
    pub is_in_european_union: Option<bool>,
    pub iso_code: Option<String>,
    pub names: Option<HashMap<String, String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RepresentedCountry {
    pub geoname_id: Option<i64>,
    pub iso_code: Option<String>,
    pub names: Option<HashMap<String, String>>,
    pub r#type: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Subdivision {
    pub confidence: Option<i64>,
    pub geoname_id: Option<i64>,
    pub iso_code: Option<String>,
    pub names: Option<HashMap<String, String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Traits {
    pub autonomous_system_number: Option<i64>,
    pub autonomous_system_organization: Option<String>,
    pub domain: Option<String>,
    pub is_anonymous: Option<bool>,
    pub is_anonymous_proxy: Option<bool>,
    pub is_anonymous_vpn: Option<bool>,
    pub is_hosting_provider: Option<bool>,
    pub is_public_proxy: Option<bool>,
    pub is_residential_proxy: Option<bool>,
    pub is_satellite_provider: Option<bool>,
    pub is_tor_exit_node: Option<bool>,
    pub isp: Option<String>,
    pub network: Option<String>,
    pub ip_address: Option<String>,
    pub organization: Option<String>,
    pub static_ip_score: Option<f64>,
    pub user_count: Option<i64>,
    pub user_type: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct MaxMind {
    pub queries_remaining: Option<i64>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Response {
    city: Option<City>,
    continent: Option<Continent>,
    country: Option<Country>,
    location: Option<Location>,
    postal: Option<Postal>,
    registered_country: Option<RegisteredCountry>,
    represented_country: Option<RepresentedCountry>,
    subdivisions: Option<Vec<Subdivision>>,
    traits: Option<Traits>,
    maxmind: Option<MaxMind>,
}
