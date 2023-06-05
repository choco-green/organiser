DATABASE_URL="postgres://superuser:superpassword@localhost/organiser"
DOCKER_CONTAINER_NAME=postgres
	
migrate-up:
	sea-orm-cli migrate -u ${DATABASE_URL} up

migrate-revert:
	sea-orm-cli migrate -u ${DATABASE_URL} down

migrate-restart:
	sea-orm-cli migrate -u ${DATABASE_URL} fresh

entity:
	sea-orm-cli generate entity -u ${DATABASE_URL} -o src/entity

docker-init:
	docker run --name ${DOCKER_CONTAINER_NAME} -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres:15
	docker exec -it ${DOCKER_CONTAINER_NAME} createdb --username=root --owner=root organiser

docker-start:
	docker start ${DOCKER_CONTAINER_NAME}