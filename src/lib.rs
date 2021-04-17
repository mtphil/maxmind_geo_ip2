use std::collections::HashMap;

struct ApiClient {
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

    #[tokio::main]
    async fn make_request(
        &self,
        prefix: String,
        ip_address: String,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}/{}", prefix, ip_address);

        let resp = self.client.get(url)
            .basic_auth(&self.user_id, Some(&self.license_key))
            .send()
            .await?
            .json::<Response>()
            .await?;
        Ok(resp)
    }

    pub fn get_country(&self, ip_address: String) -> Result<Response, Box<dyn std::error::Error>> {
        self.make_request(
            String::from("https://geoip.maxmind.com/geoip/v2.1/country/"),
            ip_address,
        )
    }

    pub fn get_city(&self, ip_address: String) -> Result<Response, Box<dyn std::error::Error>> {
        self.make_request(
            String::from("https://geoip.maxmind.com/geoip/v2.1/city/"),
            ip_address,
        )
    }

    pub fn get_insights(&self, ip_address: String) -> Result<Response, Box<dyn std::error::Error>> {
        self.make_request(
            String::from("https://geoip.maxmind.com/geoip/v2.1/insights/"),
            ip_address,
        )
    }
}

#[derive(serde::Deserialize)]
pub struct City {
    pub confidence: Option<i64>,
    pub geno_name_id: Option<i64>,
    pub names: Option<HashMap<String, String>>
}

#[derive(serde::Deserialize)]
pub struct Continent {
    pub code: Option<String>,
    pub geno_name_id: Option<i64>,
    pub names: Option<HashMap<String, String>>
}

#[derive(serde::Deserialize)]
pub struct Country {
    pub confidence: Option<i64>,
    pub geo_name_id: Option<i64>,
    pub iso_code: Option<String>,
    pub names: Option<HashMap<String, String>>
}

#[derive(serde::Deserialize)]
pub struct Location {
    pub accuracy_radius: Option<i64>,
    pub average_income: Option<i64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub metro_code: Option<i64>,
    pub population_density: Option<i64>,
    pub time_zone: Option<String>
}

#[derive(serde::Deserialize)]
pub struct Postal{
    pub code: Option<String>,
    pub confidence: Option<i64>
}

#[derive(serde::Deserialize)]
pub struct RegisteredCountry {
    pub geo_name_id: Option<i64>,
    pub iso_code: Option<String>,
    pub names: Option<HashMap<String, String>>
}

#[derive(serde::Deserialize)]
pub struct RepresentedCountry {
    pub geo_name_id: Option<i64>,
    pub iso_code: Option<String>,
    pub names: Option<HashMap<String, String>>,
    pub r#type: Option<String>
}

#[derive(serde::Deserialize)]
pub struct Subdivision {
    pub confidence: Option<i64>,
    pub geo_name_id: Option<i64>,
    pub iso_code: Option<String>,
    pub names: Option<HashMap<String, String>>
}

#[derive(serde::Deserialize)]
pub struct Traits {
    pub autonomous_system_number: Option<i64>,
    pub autonomous_system_organization: Option<String>,
    pub domain: Option<String>,
    pub is_anonymous_proxy: Option<bool>,
    pub is_satellite_provider: Option<bool>,
    pub isp: Option<String>,
    pub ip_address: Option<String>,
    pub organization: Option<String>,
    pub user_type: Option<String>
}

#[derive(serde::Deserialize)]
pub struct MaxMind {
    pub queries_remaining: Option<i64>
}

#[derive(serde::Deserialize)]
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
    maxmind: Option<MaxMind>
}
