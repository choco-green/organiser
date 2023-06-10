use actix_web::{
    error,
    web::{Data, Path, Query},
    Error, HttpResponse,
};
use sea_orm::{
    prelude::DateTimeWithTimeZone, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder,
    QueryTrait,
};
use serde::Deserialize;

use crate::{
    api::response_handler::{handle_error, handle_success},
    entity::user::{Column as UserColumn, Entity as UserEntity, Model as UserModel},
    AppState,
};

#[derive(Default, Deserialize, Clone)]
pub struct Filter {
    user_name: Option<String>,
    user_username: Option<String>,
    user_mobile: Option<String>,
    user_email: Option<String>,
    user_verified: Option<bool>,
    user_created_at_start: Option<DateTimeWithTimeZone>,
    user_created_at_end: Option<DateTimeWithTimeZone>,
    user_updated_at_start: Option<DateTimeWithTimeZone>,
    user_updated_at_end: Option<DateTimeWithTimeZone>,
}

pub async fn find_one_by_id(
    user_id: Option<Path<i32>>,
    data: Data<AppState>,
) -> Result<UserModel, Error> {
    // Parse the user_id from the request
    let user_id = match user_id {
        Some(user_id) => user_id.into_inner(),
        None => return Err(error::ErrorBadRequest("user id failed to parse")),
    };

    // Find the user model by the user_id
    let user = match UserEntity::find_by_id(user_id).one(&data.db).await {
        Ok(user) => match user {
            Some(user) => user,
            None => return Err(error::ErrorNotFound("user not found")),
        },
        Err(DbErr) => return Err(error::ErrorInternalServerError(DbErr)),
    };

    Ok(user)
}

pub async fn find_one_by_query(
    query: Option<Query<Filter>>,
    data: Data<AppState>,
) -> Result<UserModel, Error> {
    let query = match query {
        Some(query) => query.into_inner(),
        None => return Err(error::ErrorBadRequest("user not found")),
    };

    let user = match UserEntity::find()
        .filter(generate_filter(query))
        .one(&data.db)
        .await
    {
        Ok(user) => match user {
            Some(user) => user,
            None => return Err(error::ErrorNotFound("user not found")),
        },
        Err(DbErr) => return Err(error::ErrorInternalServerError(DbErr)),
    };

    Ok(user)
}

pub async fn find_many_by_query(
    query: Option<Query<Filter>>,
    data: Data<AppState>,
) -> Result<Vec<UserModel>, Error> {
    let query = match query {
        Some(query) => query.into_inner(),
        None => return Err(error::ErrorBadRequest("user not found")),
    };

    let users = match UserEntity::find()
        .filter(generate_filter(query))
        .order_by_asc(UserColumn::UserId)
        .all(&data.db)
        .await
    {
        Ok(users) => users,
        Err(DbErr) => return Err(error::ErrorInternalServerError(DbErr)),
    };

    Ok(users)
}

fn generate_filter(query: Filter) -> Condition {
    let mut conditions: Condition = Condition::all();
    if let Some(user_name) = query.user_name {
        conditions = conditions.add(UserColumn::UserName.eq(user_name));
    }
    if let Some(user_mobile) = query.user_mobile {
        conditions = conditions.add(UserColumn::UserMobile.eq(user_mobile));
    }
    if let Some(user_email) = query.user_email {
        conditions = conditions.add(UserColumn::UserEmail.eq(user_email));
    }
    if let Some(user_verified) = query.user_verified {
        conditions = conditions.add(UserColumn::UserVerified.eq(user_verified));
    }
    if let Some(user_created_at_start) = query.user_created_at_start {
        conditions = conditions.add(UserColumn::UserCreatedAt.gt(user_created_at_start));
    }
    if let Some(user_created_at_end) = query.user_created_at_end {
        conditions = conditions.add(UserColumn::UserCreatedAt.lt(user_created_at_end));
    }
    if let Some(user_updated_at_start) = query.user_updated_at_start {
        conditions = conditions.add(UserColumn::UserUpdatedAt.gt(user_updated_at_start));
    }
    if let Some(user_updated_at_end) = query.user_updated_at_end {
        conditions = conditions.add(UserColumn::UserUpdatedAt.lt(user_updated_at_end));
    }

    conditions
}
