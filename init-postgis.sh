#!/bin/bash
set -e



# Start PostgreSQL with PostGIS using docker run command
docker run -d postgis/postgis    --name geojsondb     -e POSTGRES_USER=test     -e POSTGRES_PASSWORD=admin     -e POSTGRES_DB=geojson-store     -p 5432:5432 

until psql -h "localhost" -U "test" -p 5432 -d "geojson-store" -c '\q' ; do
	>&2 echo \"Postgres is still unavailable - sleeping\"
	sleep 1
done

psql -v ON_ERROR_STOP=1 --username "test" --password "admin" <<-EOSQL
    CREATE EXTENSION IF NOT EXISTS postgis;
    CREATE EXTENSION IF NOT EXISTS postgis_topology;
EOSQL

# Get the IP address of the running container
IP_ADDRESS=

# Print the database URL in the console
echo "Database URL: postgres://test:admin@:5432/geojson-store"

echo "PostgreSQL with PostGIS is up and running!"
