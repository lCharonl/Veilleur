use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use thiserror::Error;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertEvent {
    pub serial_number: String,
    pub signature_algo: String,
    pub issuer_country: Option<String>,
    pub issuer_organisation: Option<String>,
    pub issuer_common_name: String,
    pub validity_not_before: DateTime<Utc>,
    pub validity_not_after: DateTime<Utc>,
    pub domains: Vec<String>,
    pub source: String,
}

#[async_trait]
pub trait CtFetcher: Send + Sync {
    //log name
    fn name(&self) -> &str;

    //actual tree sisze
    async fn tree_size(&self) -> Result<u64, FetchError>;

    async fn fetch_batch(&self, start:u64, end:u64) -> Result<Vec<CertEvent>, FetchError>;
}
