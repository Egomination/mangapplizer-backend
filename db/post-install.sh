#!/bin/sh

git clone https://github.com/postgrespro/rum.git

su - postgres
psql -h localhost -p 5432 -U postgres -a -q -f /crole.sql

cd /rum/
make USE_PGXS=1
make USE_PGXS=1 install
make USE_PGXS=1 installcheck
psql postgres -c "CREATE EXTENSION rum;" 
