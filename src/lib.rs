use std::collections::HashMap;
use std::fmt;
use reqwest_wasm_ext::ReqwestExt;
use reqwest::Client as HttpClient;

// divide into mods
pub struct ApiClient {
    user_id: String,
    license_key: String,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new_client(user_id: &str, license_key: &str) -> ApiClient {
        let client = HttpClient::new();
        let user_id = user_id.to_string();
        let license_key = license_key.to_string();
        ApiClient {
            user_id,
            license_key,
            client,
        }
    }

    pub async fn get_country(
        &self,
        ip_address: &str,
    ) -> Result<CountryResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "https://geoip.maxmind.com/geoip/v2.1/country/{}",
            ip_address
        );

        let resp = self
            .client
            .get(&url)
            .basic_auth_ext(&self.user_id, Some(&self.license_key))
            .send()
            .await?;

        println!("raw response{:?}", resp);

        let json_resp = resp.json::<CountryResponse>().await?;
        Ok(json_resp)
    }

    pub async fn get_city(
        &self,
        ip_address: &str,
    ) -> Result<CityResponse, Box<dyn std::error::Error>> {
        let url = format!("https://geoip.maxmind.com/geoip/v2.1/city/{}", ip_address);

        let resp = self
            .client
            .get(&url)
            .basic_auth_ext(&self.user_id, Some(&self.license_key))
            .send()
            .await?
            .json::<CityResponse>()
            .await?;

        Ok(resp)
    }

    pub async fn get_insights(
        &self,
        ip_address: &str,
    ) -> Result<InsightsResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "https://geoip.maxmind.com/geoip/v2.1/insights/{}",
            ip_address
        );

        let resp = self
            .client
            .get(&url)
            .basic_auth_ext(&self.user_id, Some(&self.license_key))
            .send()
            .await?
            .json::<InsightsResponse>()
            .await?;
        Ok(resp)
    }
}

fn option_to_string<T: fmt::Debug>(option: &Option<T>) -> String {
    match option {
        Some(t) => format!("{:?}", t).to_string(),
        None => "".to_string(),
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct City {
    pub confidence: Option<i64>,
    pub geoname_id: i64,
    pub names: HashMap<String, String>,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "confidence: {}, geoname_id: {}, names: {:?}",
            option_to_string(&self.confidence),
            &self.geoname_id,
            &self.names
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct Continent {
    pub code: String,
    pub geoname_id: i64,
    pub names: HashMap<String, String>,
}

impl fmt::Display for Continent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "code: {}, geoname_id: {}, names: {:?}",
            &self.code, &self.geoname_id, &self.names
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct Country {
    pub confidence: Option<i64>,
    pub geoname_id: i64,
    pub is_in_european_union: Option<bool>,
    pub iso_code: String,
    pub names: HashMap<String, String>,
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "confidence: {}, geoname_id: {}, is_in_european_union: {}, iso_code: {}, names: {:?}",
            option_to_string(&self.confidence),
            &self.geoname_id,
            option_to_string(&self.is_in_european_union),
            &self.iso_code,
            &self.names
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct Location {
    pub accuracy_radius: i64,
    pub average_income: Option<i64>,
    pub latitude: f64,
    pub longitude: f64,
    pub metro_code: i64,
    pub population_density: Option<i64>,
    pub time_zone: String,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "accuracy_radius: {}, average_income: {}, latitude: {}, longitude: {}, metro_code: {}, population_density: {}, time_zone: {}",
            &self.accuracy_radius,
            option_to_string(&self.average_income),
            &self.latitude,
            &self.longitude,
            &self.metro_code,
            option_to_string(&self.population_density),
            &self.time_zone
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct Postal {
    pub code: String,
    pub confidence: Option<i64>,
}

impl fmt::Display for Postal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "code: {}, confidence: {}",
            &self.code,
            option_to_string(&self.confidence)
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct RegisteredCountry {
    pub geoname_id: i64,
    pub is_in_european_union: Option<bool>,
    pub iso_code: String,
    pub names: HashMap<String, String>,
}

impl fmt::Display for RegisteredCountry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "geoname_id: {}, is_in_european_union: {}, iso_code: {}, names: {:?}",
            &self.geoname_id,
            option_to_string(&self.is_in_european_union),
            &self.iso_code,
            &self.names
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct RepresentedCountry {
    pub geoname_id: i64,
    pub is_in_european_union: Option<bool>,
    pub iso_code: String,
    pub names: HashMap<String, String>,
    pub r#type: String,
}

impl fmt::Display for RepresentedCountry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "geoname_id: {}, is_in_european_union: {}, iso_code: {}, names: {:?}, type: {}",
            &self.geoname_id,
            option_to_string(&self.is_in_european_union),
            &self.iso_code,
            &self.names,
            &self.r#type
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct Subdivision {
    pub confidence: Option<i64>,
    pub geoname_id: i64,
    pub iso_code: String,
    pub names: HashMap<String, String>,
}

impl fmt::Display for Subdivision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "confidence: {}, geoname_id: {}, iso_code: {}, names: {:?}",
            option_to_string(&self.confidence),
            &self.geoname_id,
            &self.iso_code,
            &self.names
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
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
    pub network: String,
    pub ip_address: String,
    pub organization: Option<String>,
    pub static_ip_score: Option<f64>,
    pub user_count: Option<i64>,
    pub user_type: Option<String>,
}

impl fmt::Display for Traits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "autonomous_system_number: {}, autonomous_system_organization: {}, domain: {}, is_anonymous: {}, is_anonymous_proxy:{}, is_anonymous_vpn: {}, is_hosting_provider: {}, is_public_proxy: {}, is_residential_proxy: {}, is_satellite_provider: {}, is_tor_exit_node: {}, isp: {}, network: {}, ip_address: {}, organization: {}, static_ip_score: {}, user_count: {}, user_type: {}",
            option_to_string(&self.autonomous_system_number),
            option_to_string(&self.autonomous_system_organization),
            option_to_string(&self.domain),
            option_to_string(&self.is_anonymous),
            option_to_string(&self.is_anonymous_proxy),
            option_to_string(&self.is_anonymous_vpn),
            option_to_string(&self.is_hosting_provider),
            option_to_string(&self.is_public_proxy),
            option_to_string(&self.is_residential_proxy),
            option_to_string(&self.is_satellite_provider),
            option_to_string(&self.is_tor_exit_node),
            option_to_string(&self.isp),
            &self.network,
            &self.ip_address,
            option_to_string(&self.organization),
            option_to_string(&self.static_ip_score),
            option_to_string(&self.user_count),
            option_to_string(&self.user_type)
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct MaxMind {
    pub queries_remaining: i64,
}

impl fmt::Display for MaxMind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "queries_remaining: {}", &self.queries_remaining)
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct CountryResponse {
    pub continent: Continent,
    pub maxmind: MaxMind,
    pub traits: Traits,
    pub represented_country: Option<RepresentedCountry>,
    pub registered_country: RegisteredCountry,
    pub country: Country,
}

impl fmt::Display for CountryResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "continent: {}, country: {}, registered_country: {}, represented_country: {}, traits: {}, maxmind: {}",
            &self.continent,
            &self.country,
            &self.registered_country,
            option_to_string(&self.represented_country),
            &self.traits,
            &self.maxmind
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct CityResponse {
    pub city: City,
    pub continent: Continent,
    pub country: Country,
    pub location: Location,
    pub postal: Postal,
    pub represented_country: Option<RepresentedCountry>,
    pub registered_country: RegisteredCountry,
    pub subdivisions: Vec<Subdivision>,
    pub traits: Traits,
    pub maxmind: MaxMind,
}

impl fmt::Display for CityResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "city: {}, continent: {}, country: {}, location: {}, postal: {}, registered_country: {}, represented_country: {}, subdivisions: {:?}, traits: {}, maxmind: {}",
            &self.city,
            &self.continent,
            &self.country,
            &self.location,
            &self.postal,
            &self.registered_country,
            option_to_string(&self.represented_country),
            &self.subdivisions,
            &self.traits,
            &self.maxmind
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct InsightsResponse {
    pub city: City,
    pub continent: Continent,
    pub country: Country,
    pub location: Location,
    pub postal: Postal,
    pub represented_country: Option<RepresentedCountry>,
    pub registered_country: RegisteredCountry,
    pub subdivisions: Vec<Subdivision>,
    pub traits: Traits,
    pub maxmind: MaxMind,
}

impl fmt::Display for InsightsResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "city: {}, continent: {}, country: {}, location: {}, postal: {}, registered_country: {}, represented_country: {}, subdivisions: {:?}, traits: {}, maxmind: {}",
            &self.city,
            &self.continent,
            &self.country,
            &self.location,
            &self.postal,
            &self.registered_country,
            option_to_string(&self.represented_country),
            &self.subdivisions,
            &self.traits,
            &self.maxmind
        )
    }
}
