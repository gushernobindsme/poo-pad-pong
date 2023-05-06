use crate::error::DomainError;
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Object {
    pub id: String,
    pub attributes: HashMap<String, String>,
}

#[async_trait]
pub trait ObjectRepository: Clone + Send + Sync + 'static {
    async fn find_all(&self) -> Result<Vec<Object>, DomainError>;
    async fn get(&self, id: String) -> Result<Option<Object>, DomainError>;
    async fn create(&self, attributes: HashMap<String, String>) -> Result<Object, DomainError>;
    async fn update(
        &self,
        id: String,
        attributes: HashMap<String, String>,
    ) -> Result<Object, DomainError>;
    async fn delete(&self, id: String) -> Result<(), DomainError>;
}
