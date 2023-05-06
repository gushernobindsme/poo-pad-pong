use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::EnumIter;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Enums
        manager
            .create_type(
                Type::create()
                    .as_enum(GenerationType::Table)
                    .values([GenerationType::Equals, GenerationType::Regex])
                    .to_owned(),
            )
            .await?;

        // Objects
        manager
            .create_table(
                Table::create()
                    .table(Objects::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Objects::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Objects::Attributes).json_binary().not_null())
                    .col(
                        ColumnDef::new(Objects::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Objects::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Fields
        manager
            .create_table(
                Table::create()
                    .table(Fields::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Fields::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Fields::DataLabel).string().not_null())
                    .col(ColumnDef::new(Fields::Label).string().not_null())
                    .col(
                        ColumnDef::new(Fields::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Fields::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Rules
        manager
            .create_table(
                Table::create()
                    .table(Rules::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Rules::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Rules::FieldId).string().not_null())
                    .col(
                        ColumnDef::new(Rules::Type)
                            .enumeration(
                                GenerationType::Table,
                                [GenerationType::Equals, GenerationType::Regex],
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(Rules::RegexPattern).string())
                    .col(ColumnDef::new(Rules::RegexReplacer).string())
                    .col(
                        ColumnDef::new(Rules::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Rules::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("rules_fk_fields")
                            .from(Rules::Table, Rules::FieldId)
                            .to(Fields::Table, Fields::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Keys
        manager
            .create_table(
                Table::create()
                    .table(Keys::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Keys::RuleId).string().not_null())
                    .col(ColumnDef::new(Keys::ObjectId).string().not_null())
                    .col(ColumnDef::new(Keys::Key).string().not_null())
                    .col(
                        ColumnDef::new(Keys::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Keys::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("keys_pkey")
                            .col(Keys::RuleId)
                            .col(Keys::ObjectId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("keys_fk_rules")
                            .from(Keys::Table, Keys::RuleId)
                            .to(Rules::Table, Rules::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("keys_fk_objects")
                            .from(Keys::Table, Keys::ObjectId)
                            .to(Objects::Table, Objects::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Objects::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Fields::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Rules::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Keys::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Objects {
    Table,
    Id,
    Attributes,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Fields {
    Table,
    Id,
    DataLabel,
    Label,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Rules {
    Table,
    Id,
    FieldId,
    Type,
    RegexPattern,
    RegexReplacer,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden, EnumIter)]
pub enum GenerationType {
    Table,
    #[iden = "Equals"]
    Equals,
    #[iden = "Regex"]
    Regex,
}

#[derive(Iden)]
enum Keys {
    Table,
    RuleId,
    ObjectId,
    Key,
    CreatedAt,
    UpdatedAt,
}
