#!/bin/bash

# Variables for PostgreSQL configuration
POSTGRES_USER=test
POSTGRES_PASSWORD=admin
POSTGRES_DB=geojson-store


# Start PostgreSQL with PostGIS using docker run command
docker run \
    --name geojsondb \
    -e POSTGRES_USER=${POSTGRES_USER} \
    -e POSTGRES_PASSWORD=${POSTGRES_PASSWORD} \
    -e POSTGRES_DB=${POSTGRES_DB} \
    -p 5432:5432 \
    postgis/postgis

until psql -h "localhost" -U "${POSTGRES_USER}" -p 5432 -d "${POSTGRES_DB}" -c '\q' ; do
	>&2 echo \"Postgres is still unavailable - sleeping\"
	sleep 1
done

# Get the IP address of the running container
IP_ADDRESS=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' geojsondb)

# Print the database URL in the console
echo "Database URL: postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$IP_ADDRESS:5432/$POSTGRES_DB"

echo "PostgreSQL with PostGIS is up and running!"
