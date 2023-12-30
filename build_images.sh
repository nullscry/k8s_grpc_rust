#!/bin/sh
cd clock_tonic
docker build --no-cache -t clock_image .
cd ../numbers
docker build --no-cache -t numbers_image .
cd ../db_api
docker build --no-cache -t dbapi_image .
cd ..