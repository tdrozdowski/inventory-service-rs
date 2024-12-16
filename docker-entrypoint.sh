#!/bin/bash
set -e

echo "Starting migration process..."

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
  echo "Error: DATABASE_URL is not set."
  exit 1
fi

# Debugging info (optional)
echo "Using DATABASE_URL: $DATABASE_URL"

# Run sqlx migrations (update this command as needed for your use case)
sqlx database setup

# If you want to open a custom shell instead after migrations
exec "$@"