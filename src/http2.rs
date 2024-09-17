#[cfg(feature = "blocking")]
pub type Response = reqwest::blocking::Response;

#[cfg(not(feature = "blocking"))]
pub type Response = reqwest::Response;

#[cfg(feature = "blocking")]
pub type Client = reqwest::blocking::Client;

#[cfg(not(feature = "blocking"))]
pub type Client = reqwest::Client;
