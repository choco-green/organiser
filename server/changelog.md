# Server Changelog


## [0.0.1] - 05-06-2023
### Added 
- Endpoint for Calendar:
    - GET /calendar/{calendarId}
        - This gets the calendar by the calendar id, no matter who the calendar is owned by
    - GET /calendar/user/{userId}
        - This gets all the calendar owned by the user
- Finalising the crates to use and database to use
    - Using an ORM to handle query (sea-orm)
    - Using serde and serde_json to handle JSON
    - Using actix-web to handle HTTP responses
    - All database migration is handled with sea-orm-migration
    - Using postgres for database
    - Using docker to host locally
- Makefile is created
    - Apply all migrations `make migrate-up`
    - Revert last migration `make migrate-revert`
    - Drop database and then apply all migrations `make migrate-restart`
    - Generate entities for the database `make entity`
    - Initialise docker container `make docker-init`
    - Start docker container `make docker-start`
