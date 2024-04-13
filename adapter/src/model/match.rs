//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::sea_orm_active_enums::Status;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "match")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub title: String,
    pub channel_id: String,
    pub message_id: String,
    pub created_at: DateTimeUtc,
    pub deadline_at: DateTimeUtc,
    pub status: Status,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::bet::Entity")]
    Bet,
    #[sea_orm(has_many = "super::candidate::Entity")]
    Candidate,
}

impl Related<super::bet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bet.def()
    }
}

impl Related<super::candidate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Candidate.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
