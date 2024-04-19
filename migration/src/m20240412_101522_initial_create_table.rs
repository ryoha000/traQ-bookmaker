use sea_orm_migration::prelude::*;

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
        // traq_id と channel_id に複合ユニーク制約を追加
        manager.get_connection().execute_unprepared("ALTER TABLE user ADD CONSTRAINT unique_user_traq_id_channel_id UNIQUE (traq_id, channel_id)").await?;

        manager
            .create_table(
                Table::create()
                    .table(Match::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Match::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Match::Title).string().not_null())
                    .col(ColumnDef::new(Match::ChannelId).string().not_null())
                    .col(ColumnDef::new(Match::MessageId).string().null())
                    .col(
                        ColumnDef::new(Match::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Match::ClosedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Match::FinishedAt)
                            .timestamp_with_time_zone()
                            .null(),
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
        // match_id と name に複合ユニーク制約を追加
        manager.get_connection().execute_unprepared("ALTER TABLE candidate ADD CONSTRAINT unique_candidate_match_id_name UNIQUE (match_id, name)").await?;

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
        // match_id と user_id に複合ユニーク制約を追加
        manager.get_connection().execute_unprepared("ALTER TABLE bet ADD CONSTRAINT unique_bet_match_id_user_id UNIQUE (match_id, user_id)").await?;

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
    ChannelId,
    MessageId,
    CreatedAt,
    ClosedAt,
    FinishedAt,
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
