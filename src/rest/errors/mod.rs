// Error types for the REST API
// Currently using anyhow::Result for error handling
// Custom error types can be added here as needed

include!(concat!(env!("OUT_DIR"), "/error_codes.rs"));
