use sea_orm_migration::{prelude::*, schema::*};

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
                    .col(integer(User::Id)
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(string(User::Email).unique_key().not_null())
                    .col(string(User::Password).not_null())
                    .col(string(User::Firstname).null())
                    .col(string(User::Lastname).null())
                    .col(timestamp(User::CreatedAt)
                        .extra("DEFAULT CURRENT_TIMESTAMP")
                        .to_owned()
                    )
                    .col(timestamp(User::UpdatedAt)
                        .extra("DEFAULT CURRENT_TIMESTAMP")
                        .to_owned()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Email,
    Password,
    Firstname,
    Lastname,
    UpdatedAt,
    CreatedAt,
}
