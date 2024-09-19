use sea_orm_migration::{prelude::*, schema::*};
use crate::migrator::m20220101_000001_create_user_table::User;
use crate::migrator::m20240917_074218_create_author_table::Author;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .if_not_exists()
                    .col(
                        integer(Book::Id)
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        integer(Book::AuthorId)
                            .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk-book-author_id")
                        .from(Book::Table, Book::AuthorId)
                        .to(Author::Table, Author::Id)
                    )
                    .col(string(Book::Title).not_null())
                    .col(string(Book::Year).not_null())
                    .col(string(Book::Cover).not_null())
                    .col(timestamp(Book::CreatedAt)
                        .extra("DEFAULT CURRENT_TIMESTAMP")
                        .to_owned()
                    )
                    .col(timestamp(Book::UpdatedAt)
                        .extra("DEFAULT CURRENT_TIMESTAMP")
                        .to_owned()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Book {
    Table,
    Id,
    AuthorId,
    Title,
    Year,
    Cover,
    CreatedAt,
    UpdatedAt
}
