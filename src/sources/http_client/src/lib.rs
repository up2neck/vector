//#[cfg(feature = "sources-http_client")]
pub mod client;

#[cfg(test)]
mod tests;

#[cfg(all(test, feature = "http-client-integration-tests"))]
mod integration_tests;

//#[cfg(feature = "sources-http_client")]
pub use client::HttpClientConfig;
