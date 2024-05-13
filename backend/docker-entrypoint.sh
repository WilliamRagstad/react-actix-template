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

#*****************************************************************
#*                                                               *
#*  Perform any database migrations or other startup tasks here  *
#*                                                               *
#*****************************************************************

# Start the main application
echo "[*] Starting the application..."
exec "$@"
