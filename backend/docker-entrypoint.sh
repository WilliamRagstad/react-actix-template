#!/bin/bash
echo "[*] Starting the backend container entrypoint script..."

ls -la

set -o allexport
. .env
set +o allexport

# Wait for the database to be ready
until mysqladmin ping -h "mysql_db" --silent; do
	echo "[*] Waiting for mysql_db..."
	sleep 1
done
echo "[*] mysql_db is ready!"

# Run Diesel migrations
echo "[*] Running migrations..."
diesel migration run
# Or diesel setup

# Start the main application
echo "[*] Starting the application..."
exec "$@"
