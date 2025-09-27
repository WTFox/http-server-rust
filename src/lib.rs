pub mod appconfig;
pub mod request;
pub mod response;
pub mod routes;

use std::collections::HashMap;

pub use appconfig::AppConfig;

pub use request::HttpMethod;
pub use request::Request;
pub use response::Response;

pub type Headers = HashMap<String, String>;
