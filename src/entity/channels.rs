//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "channels")]
pub struct Model {
    #[sea_orm(column_type = "custom(\"TINYTEXT\")", nullable)]
    pub prefix: Option<String>,
    #[sea_orm(column_type = "custom(\"TINYTEXT\")", nullable)]
    pub locale: Option<String>,
    pub notify: Option<i32>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
