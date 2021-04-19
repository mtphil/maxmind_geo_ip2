# maxmind_geo_ip2 API client

Full documentation here: [https://docs.rs/maxmind_geo_ip2/](https://docs.rs/maxmind_geo_ip2/)

Crates.io page here: [https://crates.io/crates/maxmind_geo_ip2](https://crates.io/crates/maxmind_geo_ip2)

WARNING: This is still very much a ***work in progress*** - this should probably/definitely NOT BE USED IN PRODUCTION.

This is my first Rust crate, so any suggestion about improvements, etc are very welcome.

To-dos: 
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
get_country(ip_address: &str) -> Result<CountryResponse, Box<dyn std::error::Error>>
````

Response:

````rust
pub struct CountryResponse {
    pub continent: Continent,
    pub maxmind: MaxMind,
    pub traits: Traits,
    pub represented_country: Option<RepresentedCountry>,
    pub registered_country: RegisteredCountry,
    pub country: Country,
}
````

### Get City

API endpoint:
````
https://geoip.maxmind.com/geoip/v2.1/city/{ip_address}
````

Method signature:
````rust
get_city(ip_address: &str) -> Result<CityResponse, Box<dyn std::error::Error>>
````

Response: 

````rust
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
````

### Get Insights

API endpoint:
````
https://geoip.maxmind.com/geoip/v2.1/insights/{ip_address}
```

Method signature:
````rust
get_insights(ip_address: &str) -> Result<InsightsResponse, Box<dyn std::error::Error>>
````

Response:

````rust
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
````