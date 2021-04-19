use std::collections::HashMap;
use std::fmt;
// TODO - make three different response objects - eliminate unnecessary Option<>s
pub struct ApiClient {
    user_id: String,
    license_key: String,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new_client(user_id: &str, license_key: &str) -> ApiClient {
        let client = reqwest::Client::new();
        let user_id = user_id.to_string();
        let license_key = license_key.to_string();
        ApiClient {
            user_id,
            license_key,
            client,
        }
    }

    async fn make_request(
        &self,
        prefix: String,
        ip_address: &str,
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
        ip_address: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        self.make_request(
            String::from("https://geoip.maxmind.com/geoip/v2.1/country/"),
            ip_address,
        )
        .await
    }

    pub async fn get_city(&self, ip_address: &str) -> Result<Response, Box<dyn std::error::Error>> {
        self.make_request(
            String::from("https://geoip.maxmind.com/geoip/v2.1/city/"),
            ip_address,
        )
        .await
    }

    pub async fn get_insights(
        &self,
        ip_address: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        self.make_request(
            String::from("https://geoip.maxmind.com/geoip/v2.1/insights/"),
            ip_address,
        )
        .await
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
    pub geoname_id: Option<i64>,
    pub names: Option<HashMap<String, String>>,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "confidence: {}, geoname_id: {}, names: {}",
            option_to_string(&self.confidence),
            option_to_string(&self.geoname_id),
            option_to_string(&self.names)
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct Continent {
    pub code: Option<String>,
    pub geoname_id: Option<i64>,
    pub names: Option<HashMap<String, String>>,
}

impl fmt::Display for Continent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "code: {}, geoname_id: {}, names: {}",
            option_to_string(&self.code),
            option_to_string(&self.geoname_id),
            option_to_string(&self.names)
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct Country {
    pub confidence: Option<i64>,
    pub geoname_id: Option<i64>,
    pub is_in_european_union: Option<bool>,
    pub iso_code: Option<String>,
    pub names: Option<HashMap<String, String>>,
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "confidence: {}, geoname_id: {}, is_in_european_union: {}, iso_code: {}, names: {}",
            option_to_string(&self.confidence),
            option_to_string(&self.geoname_id),
            option_to_string(&self.is_in_european_union),
            option_to_string(&self.iso_code),
            option_to_string(&self.names)
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct Location {
    pub accuracy_radius: Option<i64>,
    pub average_income: Option<i64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub metro_code: Option<i64>,
    pub population_density: Option<i64>,
    pub time_zone: Option<String>,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "accuracy_radius: {}, average_income: {}, latitude: {}, longitude: {}, metro_code: {}, population_density: {}, time_zone: {}",
            option_to_string(&self.accuracy_radius),
            option_to_string(&self.average_income),
            option_to_string(&self.latitude),
            option_to_string(&self.longitude),
            option_to_string(&self.metro_code),
            option_to_string(&self.population_density),
            option_to_string(&self.time_zone)
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct Postal {
    pub code: Option<String>,
    pub confidence: Option<i64>,
}

impl fmt::Display for Postal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "code: {}, confidence: {}",
            option_to_string(&self.code),
            option_to_string(&self.confidence)
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct RegisteredCountry {
    pub geoname_id: Option<i64>,
    pub is_in_european_union: Option<bool>,
    pub iso_code: Option<String>,
    pub names: Option<HashMap<String, String>>,
}

impl fmt::Display for RegisteredCountry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "geoname_id: {}, is_in_european_union: {}, iso_code: {}, names: {}",
            option_to_string(&self.geoname_id),
            option_to_string(&self.is_in_european_union),
            option_to_string(&self.iso_code),
            option_to_string(&self.names)
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct RepresentedCountry {
    pub geoname_id: Option<i64>,
    pub iso_code: Option<String>,
    pub names: Option<HashMap<String, String>>,
    pub r#type: Option<String>,
}

impl fmt::Display for RepresentedCountry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "geoname_id: {}, iso_code: {}, names: {}, type: {}",
            option_to_string(&self.geoname_id),
            option_to_string(&self.iso_code),
            option_to_string(&self.names),
            option_to_string(&self.r#type)
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct Subdivision {
    pub confidence: Option<i64>,
    pub geoname_id: Option<i64>,
    pub iso_code: Option<String>,
    pub names: Option<HashMap<String, String>>,
}

impl fmt::Display for Subdivision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "confidence: {}, geoname_id: {}, iso_code: {}, names: {}",
            option_to_string(&self.confidence),
            option_to_string(&self.geoname_id),
            option_to_string(&self.iso_code),
            option_to_string(&self.names)
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
    pub network: Option<String>,
    pub ip_address: Option<String>,
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
            option_to_string(&self.network),
            option_to_string(&self.ip_address),
            option_to_string(&self.organization),
            option_to_string(&self.static_ip_score),
            option_to_string(&self.user_count),
            option_to_string(&self.user_type)
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct MaxMind {
    pub queries_remaining: Option<i64>,
}

impl fmt::Display for MaxMind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "queries_remaining: {}",
            option_to_string(&self.queries_remaining)
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, fmt::Debug)]
pub struct Response {
    pub city: Option<City>,
    pub continent: Option<Continent>,
    pub country: Option<Country>,
    pub location: Option<Location>,
    pub postal: Option<Postal>,
    pub registered_country: Option<RegisteredCountry>,
    pub represented_country: Option<RepresentedCountry>,
    pub subdivisions: Option<Vec<Subdivision>>,
    pub traits: Option<Traits>,
    pub maxmind: Option<MaxMind>,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "city: {}, continent: {}, country: {}, location: {}, postal: {}, registered_country: {}, represented_country: {}, subdivisions: {}, traits: {}, maxmind: {}",
            option_to_string(&self.city),
            option_to_string(&self.continent),
            option_to_string(&self.country),
            option_to_string(&self.location),
            option_to_string(&self.postal),
            option_to_string(&self.registered_country),
            option_to_string(&self.represented_country),
            option_to_string(&self.subdivisions),
            option_to_string(&self.traits),
            option_to_string(&self.maxmind),
        )
    }
}
