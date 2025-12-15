// Main library file for the travel tech assessment

// Export modules for each part of the assessment
pub mod part1_cache;
pub mod part1_cache_example; // Example implementation for reference
pub mod part2_xml;
pub mod part2_xml_example; // Example implementation for reference
pub mod part3_api;
pub mod part3_api_example; // Example implementation for reference

// Re-export key types for convenience
pub use part1_cache::{AvailabilityCache, CacheStats};
pub use part2_xml::{HotelSearchProcessor, ProcessedResponse, HotelOption, FilterCriteria, ProcessingError};
pub use part3_api::{BookingApiClient, ApiClient, ClientConfig, ClientStats, ApiError, ClientError};
