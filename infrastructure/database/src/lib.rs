use crate::entities::fields;
use crate::entities::objects;
use crate::entities::rules;
use crate::entities::sea_orm_active_enums::GenerationType;
use anyhow::Result;
use domain::error::DomainError;
use domain::fields::Field;
use domain::objects::Object;
use domain::rules::{GenerationRule, Rule};
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::collections::HashMap;

pub mod client;
pub mod entities;

pub async fn create_database_connection(database_url: String) -> Result<DatabaseConnection, DbErr> {
    Database::connect(&database_url).await
}

impl From<fields::Model> for Field {
    fn from(value: fields::Model) -> Self {
        Field {
            id: value.id,
            data_label: value.data_label,
            label: value.label,
        }
    }
}

pub fn to_object(
    object: objects::Model,
    fields: Vec<fields::Model>,
) -> Result<Object, DomainError> {
    let mut lookup = object
        .attributes
        .as_object()
        .ok_or(DomainError::Unexpected("".to_string()))?
        .clone();
    let mut attributes = HashMap::new();
    for field in fields {
        let (k, v) = lookup
            .remove_entry(&field.data_label)
            .ok_or(DomainError::Unexpected("".to_string()))?;
        attributes.insert(k, v.to_string());
    }

    Ok(Object {
        id: object.id,
        attributes,
    })
}

pub fn to_rule(rule: rules::Model, field: fields::Model) -> Rule {
    Rule {
        id: rule.id,
        field: field.into(),
        rule_type: match rule.r#type {
            GenerationType::Equals => GenerationRule::Equals,
            GenerationType::Regex => GenerationRule::Regex {
                pattern: rule.regex_pattern.unwrap_or("".to_string()),
                replacer: rule.regex_replacer.unwrap_or("".to_string()),
            },
        },
    }
}
