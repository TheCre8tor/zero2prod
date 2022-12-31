#!/bin/bash
#! https://stackoverflow.com/questions/38624897/how-to-delete-postgresql-databases-matching-a-pattern#answer-38669981

clear
export PGPASSWORD="password"

PATTERN="TEST"

echo "Pattern parameter: $PATTERN"

psql -U alexander -d postgres -c "copy (select datname from pg_database where datname like '%$PATTERN%') to stdout" | while read line; do
    echo "$line"
    dropdb -U alexander "$line"
done
echo
echo "Databases which names matches pattern $PATTERN were deleted!"
echo
exit