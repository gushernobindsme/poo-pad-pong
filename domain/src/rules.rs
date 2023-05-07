use crate::error::DomainError;
use crate::fields::Field;
use crate::objects::Object;
use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    pub id: String,
    pub field: Field,
    pub rule_type: GenerationRule,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenerationRule {
    Equals,
    Regex { pattern: String, replacer: String },
}

impl Rule {
    pub fn generate_key(&self, object: Object) -> Result<String, DomainError> {
        match &self.rule_type {
            GenerationRule::Equals => {
                let value = object.attributes.get(&self.field.data_label).ok_or(
                    DomainError::Unexpected(format!(
                        "Field NotFound, key: [{}]",
                        &self.field.data_label
                    )),
                )?;
                Ok(value.to_string())
            }
            GenerationRule::Regex { pattern, replacer } => {
                let regex = Regex::new(pattern).unwrap();
                let raw_value = object
                    .attributes
                    .get(&self.field.data_label)
                    .ok_or(DomainError::Unexpected(format!(
                        "Field NotFound, key: [{}]",
                        &self.field.data_label
                    )))?
                    .to_string();
                let value = regex.replace_all(&raw_value, replacer);
                Ok(value.to_string())
            }
        }
    }
}

#[async_trait]
pub trait RuleRepository: Clone + Send + Sync + 'static {
    async fn find_all(&self) -> Result<Vec<Rule>, DomainError>;
    async fn get(&self, id: String) -> Result<Option<Rule>, DomainError>;
    async fn create(
        &self,
        field_id: String,
        rule_type: GenerationRule,
    ) -> Result<Rule, DomainError>;
    async fn update(
        &self,
        id: String,
        field_id: String,
        rule_type: GenerationRule,
    ) -> Result<Rule, DomainError>;
    async fn delete(&self, id: String) -> Result<(), DomainError>;
}
