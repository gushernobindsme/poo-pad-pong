use crate::error::DomainError;
use anyhow::Result;
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Key {
    pub rule_id: String,
    pub object_id: String,
    pub key: String,
}

#[async_trait]
pub trait KeyRepository: Clone + Send + Sync + 'static {
    async fn create_by_rule(
        &self,
        rule_id: String,
        keys: Vec<(String, String)>,
    ) -> Result<(), DomainError>;
    async fn update_by_rule(
        &self,
        rule_id: String,
        keys: Vec<(String, String)>,
    ) -> Result<(), DomainError>;
    async fn delete_by_rule(&self, rule_id: String) -> Result<(), DomainError>;
}
