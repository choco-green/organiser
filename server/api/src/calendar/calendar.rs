use entity;

entity::Calendar::find()
    .one(db)
    .await
    .expect("Failed to fetch calendar")
    .expect("Calendar not found")
    .into()