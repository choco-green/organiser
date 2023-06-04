use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Creates user table
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id)
                        .integer().not_null().auto_increment().primary_key()
                    )
                    .col(ColumnDef::new(User::Username)
                        .string().not_null().unique_key()
                    )
                    .col(ColumnDef::new(User::Mobile)
                        .string().not_null().unique_key()
                    )
                    .col(ColumnDef::new(User::Email)
                        .string().not_null().unique_key()
                    )
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .col(ColumnDef::new(User::UserCreatedAt).date_time().not_null())
                    .col(ColumnDef::new(User::UserUpdatedAt).date_time().not_null())
                    .to_owned()
            ).await?;

        // Creates index for user table
        manager
            .create_index(
                Index::create()
                    .name("idx_user")
                    .table(User::Table)
                    .if_not_exists()
                    .col(User::Username)
                    .col(User::Mobile)
                    .col(User::Email)
                    .to_owned()
            ).await?;

        // Creates calendar table
        manager
            .create_table(
                Table::create()
                .table(Calendar::Table)
                .if_not_exists()
                .col(ColumnDef::new(Calendar::Id)
                    .integer().not_null().auto_increment().primary_key()
                )
                .col(ColumnDef::new(Calendar::UserId).integer().not_null().unique_key())
                .col(ColumnDef::new(Calendar::Title).string().not_null())
                .col(ColumnDef::new(Calendar::Description).string())
                .col(ColumnDef::new(Calendar::DefaultEventColour)
                    .string().not_null()
                )
                .col(ColumnDef::new(Calendar::Notification)
                    .boolean().not_null().default(true)
                )
                .col(ColumnDef::new(Calendar::TimeZone).string())
                .col(ColumnDef::new(Calendar::CalendarCreatedAt).date_time().not_null())
                .col(ColumnDef::new(Calendar::CalendarUpdatedAt).date_time().not_null())
                .to_owned() 
            ).await?;
        
        // Create Event table
        manager
            .create_table(Table::create()
                .table(Event::Table)
                .if_not_exists()
                .col(ColumnDef::new(Event::Id)
                    .integer().not_null().auto_increment().primary_key()
                )
                .col(ColumnDef::new(Event::CalendarId).integer().not_null().unique_key())
                .col(ColumnDef::new(Event::Title).string().not_null())
                .col(ColumnDef::new(Event::Description).string())
                .col(ColumnDef::new(Event::Location).string())
                .col(ColumnDef::new(Event::EventColour).string().not_null())
                .col(ColumnDef::new(Event::StartTime).date_time().not_null())
                .col(ColumnDef::new(Event::EndTime).date_time().not_null())
                .col(ColumnDef::new(Event::TimeZone).string())
                .col(ColumnDef::new(Event::EventCreatedAt).date_time().not_null())
                .col(ColumnDef::new(Event::EventUpdatedAt).date_time().not_null())
                .to_owned()
            ).await?;

        // Creates calendar table foreign key
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user_id")
                    .from(Calendar::Table, Calendar::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;

        // Create Event table foreign key
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_calendar_id")
                    .from(Event::Table, Event::CalendarId)
                    .to(Calendar::Table, Calendar::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;
    
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Calendar::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    Id,
    Username,
    Mobile,
    Email,
    PasswordHash,
    UserCreatedAt,
    UserUpdatedAt,
}

#[derive(Iden)]
enum Calendar {
    Table,
    Id,
    UserId,
    Title,
    Description,
    DefaultEventColour,
    Notification,
    TimeZone,
    CalendarCreatedAt,
    CalendarUpdatedAt,
}

#[derive(Iden)]
enum Event {
    Table,
    Id,
    CalendarId,
    Title,
    Description,
    Location,
    EventColour,
    StartTime,
    EndTime,
    TimeZone,
    EventCreatedAt,
    EventUpdatedAt,
}
