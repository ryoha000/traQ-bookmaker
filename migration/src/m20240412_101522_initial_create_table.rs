use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(User::TraqId).string().not_null())
                    .col(ColumnDef::new(User::TraqDisplayId).string().not_null())
                    .col(ColumnDef::new(User::ChannelId).string().not_null())
                    .col(ColumnDef::new(User::Balance).integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Match::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Match::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Match::Title).string().not_null())
                    .col(
                        ColumnDef::new(Match::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Match::DeadlineAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Match::MatchStatus)
                            .enumeration(Alias::new("match_status"), MatchStatus::iter())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Candidate::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Candidate::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Candidate::Name).string().not_null())
                    .col(ColumnDef::new(Candidate::MatchId).string().not_null())
                    .col(ColumnDef::new(Candidate::IsWinner).boolean())
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_candidate_match_id")
                    .from_tbl(Candidate::Table)
                    .to_tbl(Match::Table)
                    .from_col(Candidate::MatchId)
                    .to_col(Match::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Bet::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Bet::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Bet::MatchId).string().not_null())
                    .col(ColumnDef::new(Bet::UserId).string().not_null())
                    .col(ColumnDef::new(Bet::CandidateId).string().not_null())
                    .col(ColumnDef::new(Bet::Amount).integer().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_bet_match_id")
                    .from_tbl(Bet::Table)
                    .to_tbl(Match::Table)
                    .from_col(Bet::MatchId)
                    .to_col(Match::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_bet_user_id")
                    .from_tbl(Bet::Table)
                    .to_tbl(User::Table)
                    .from_col(Bet::UserId)
                    .to_col(User::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_bet_candidate_id")
                    .from_tbl(Bet::Table)
                    .to_tbl(Candidate::Table)
                    .from_col(Bet::CandidateId)
                    .to_col(Candidate::Id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bet::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Candidate::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Match::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    TraqId,
    TraqDisplayId,
    ChannelId,
    Balance,
}

#[derive(DeriveIden)]
enum Match {
    Table,
    Id,
    Title,
    CreatedAt,
    DeadlineAt,
    #[sea_orm(iden = "status")]
    MatchStatus,
}

#[derive(Iden, EnumIter)]
pub enum MatchStatus {
    #[iden = "Scheduled"]
    Scheduled,
    #[iden = "OnGoing"]
    OnGoing,
    #[iden = "Finished"]
    Finished,
    #[iden = "Cancelled"]
    Cancelled,
}

#[derive(DeriveIden)]
enum Candidate {
    Table,
    Id,
    Name,
    MatchId,
    IsWinner,
}

#[derive(DeriveIden)]
enum Bet {
    Table,
    Id,
    MatchId,
    UserId,
    CandidateId,
    Amount,
}
