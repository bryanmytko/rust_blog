#!/bin/bash

sqlite3 app/db/blog.sqlite3 < app/db/schema.sql
cargo build
