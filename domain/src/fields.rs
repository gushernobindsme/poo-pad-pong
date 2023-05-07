use crate::error::DomainError;
use anyhow::Result;
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub id: String,
    pub data_label: String,
    pub label: String,
}

#[async_trait]
pub trait FieldRepository: Clone + Send + Sync + 'static {
    async fn find_all(&self) -> Result<Vec<Field>, DomainError>;
    async fn create(&self, data_label: String, label: String) -> Result<Field, DomainError>;
    async fn update(&self, id: String, label: String) -> Result<Field, DomainError>;
}
