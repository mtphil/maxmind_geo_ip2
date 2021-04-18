# maxmind_geo_ip2 API client

WARNING: This is still very much a ***work in progress*** - this should probably/definitely NOT BE USED IN PRODUCTION.

This is my first Rust crate, so any suggestion about improvements, etc are very welcome.

Todos: 
- error handling
- response status checks

## Dependencies
````
reqwest = { version = "0.11.3", features = ["rustls", "json"] }
serde = { version = "1.0.125", features = ["derive"] }
````

## Usage

Import the api client from the crate:

````
use maxmind_geo_ip2::ApiClient;
````

Initialize the client with Maxmind API credentials: 

````
let maxmind_client = ApiClient.new_client(String::from("user_id_here"), String::from("license_key_here"));
````

## Methods

### Get Country

API endpoint: 
````
https://geoip.maxmind.com/geoip/v2.1/country/{ip_address}
````

Method signature:
````rust
get_country(ip_address: String) -> Result<Response, Box<dyn std::error::Error>>
````


### Get City

API endpoint:
````
https://geoip.maxmind.com/geoip/v2.1/city/{ip_address}
````

Method signature:
````rust
get_city(ip_address: String) -> Result<Response, Box<dyn std::error::Error>>
````


### Get Insights

API endpoint:
````
https://geoip.maxmind.com/geoip/v2.1/insights/{ip_address}
````


Method signature:
````rust
get_insights(ip_address: String) -> Result<Response, Box<dyn std::error::Error>>
````

## Response Types
````rust
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
````

````rust
pub struct City {
    pub confidence: Option<i64>,
    pub geoname_id: Option<i64>,
    pub names: Option<HashMap<String, String>>,
}
````

````rust
pub struct Continent {
    pub code: Option<String>,
    pub geoname_id: Option<i64>,
    pub names: Option<HashMap<String, String>>,
}
````

````rust
pub struct Country {
    pub confidence: Option<i64>,
    pub geoname_id: Option<i64>,
    pub is_in_european_union: Option<bool>,
    pub iso_code: Option<String>,
    pub names: Option<HashMap<String, String>>,
}
````

````rust
pub struct Location {
    pub accuracy_radius: Option<i64>,
    pub average_income: Option<i64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub metro_code: Option<i64>,
    pub population_density: Option<i64>,
    pub time_zone: Option<String>,
}
````

````rust
pub struct Postal {
    pub code: Option<String>,
    pub confidence: Option<i64>,
}
````

````rust
pub struct RegisteredCountry {
    pub geoname_id: Option<i64>,
    pub is_in_european_union: Option<bool>,
    pub iso_code: Option<String>,
    pub names: Option<HashMap<String, String>>,
}
````

````rust
pub struct RepresentedCountry {
    pub geoname_id: Option<i64>,
    pub iso_code: Option<String>,
    pub names: Option<HashMap<String, String>>,
    pub r#type: Option<String>,
}
````

````rust
pub struct Subdivision {
    pub confidence: Option<i64>,
    pub geoname_id: Option<i64>,
    pub iso_code: Option<String>,
    pub names: Option<HashMap<String, String>>,
}
````

````rust
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
````

````rust
pub struct MaxMind {
    pub queries_remaining: Option<i64>,
}
````