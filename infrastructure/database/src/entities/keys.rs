//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "keys")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub rule_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub object_id: String,
    pub key: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::objects::Entity",
        from = "Column::ObjectId",
        to = "super::objects::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Objects,
    #[sea_orm(
        belongs_to = "super::rules::Entity",
        from = "Column::RuleId",
        to = "super::rules::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Rules,
}

impl Related<super::objects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Objects.def()
    }
}

impl Related<super::rules::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rules.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
