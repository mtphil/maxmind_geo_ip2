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

More documentation here: [https://docs.rs/maxmind_geo_ip2/](https://docs.rs/maxmind_geo_ip2/)