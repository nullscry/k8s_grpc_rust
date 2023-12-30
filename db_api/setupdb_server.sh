#!/bin/sh

/usr/local/bin/diesel setup
/usr/local/bin/diesel migration run
/usr/local/bin/db_api >>logs.txt 2>&1