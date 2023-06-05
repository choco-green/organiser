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
                    .col(
                        ColumnDef::new(User::UserId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::UserName)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(User::UserMobile)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(User::UserEmail)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::UserPassword).string().not_null())
                    .col(
                        ColumnDef::new(User::UserCreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UserUpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Creates index for user table
        manager
            .create_index(
                Index::create()
                    .name("idx_user")
                    .table(User::Table)
                    .if_not_exists()
                    .col(User::UserName)
                    .col(User::UserMobile)
                    .col(User::UserEmail)
                    .to_owned(),
            )
            .await?;

        // Creates calendar table
        manager
            .create_table(
                Table::create()
                    .table(Calendar::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Calendar::CalendarId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Calendar::UserId).integer().not_null())
                    .col(ColumnDef::new(Calendar::CalendarTitle).string().not_null())
                    .col(ColumnDef::new(Calendar::CalendarDescription).string())
                    .col(
                        ColumnDef::new(Calendar::CalendarDefaultEventColour)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Calendar::CalendarNotification)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Calendar::CalendarCreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Calendar::CalendarUpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Event table
        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Event::EventId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Event::CalendarId).integer().not_null())
                    .col(ColumnDef::new(Event::EventTitle).string().not_null())
                    .col(ColumnDef::new(Event::EventDescription).string())
                    .col(ColumnDef::new(Event::EventLocation).string())
                    .col(ColumnDef::new(Event::EventColour).string().not_null())
                    .col(ColumnDef::new(Event::EventStartTime).date_time().not_null())
                    .col(ColumnDef::new(Event::EventEndTime).date_time().not_null())
                    .col(
                        ColumnDef::new(Event::EventCreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Event::EventUpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Creates calendar table foreign key
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user_id")
                    .from(Calendar::Table, Calendar::UserId)
                    .to(User::Table, User::UserId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Create Event table foreign key
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_calendar_id")
                    .from(Event::Table, Event::CalendarId)
                    .to(Calendar::Table, Calendar::CalendarId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

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
    UserId,
    UserName,
    UserMobile,
    UserEmail,
    UserPassword,
    UserCreatedAt,
    UserUpdatedAt,
}

#[derive(Iden)]
enum Calendar {
    Table,
    CalendarId,
    UserId,
    CalendarTitle,
    CalendarDescription,
    CalendarDefaultEventColour,
    CalendarNotification,
    CalendarCreatedAt,
    CalendarUpdatedAt,
}

#[derive(Iden)]
enum Event {
    Table,
    EventId,
    CalendarId,
    EventTitle,
    EventDescription,
    EventLocation,
    EventColour,
    EventStartTime,
    EventEndTime,
    EventCreatedAt,
    EventUpdatedAt,
}
