use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(Users::Username)
                            .string()
                            .not_null()
                            .unique_key()
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .not_null()
                            .unique_key()
                    )
                    .col(
                        ColumnDef::new(Users::Password)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()) // Default เป็นเวลาปัจจุบัน
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    #[sea_orm(table_name = "users")]
    Table,
    #[sea_orm(column_name = "id")]
    Id,
    #[sea_orm(column_name = "username")]
    Username,
    #[sea_orm(column_name = "email")]
    Email,
    #[sea_orm(column_name = "password")]
    Password,
    #[sea_orm(column_name = "created_at")]
    CreatedAt,
}
