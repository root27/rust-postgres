# Rust Postgres

This is a simple rust actix web server that connects to a postgres database.

## Running the server

```bash

cargo run 

```

## Running the Postgres database

```bash

docker run -it -d --name some-postgres -e POSTGRES_USER=test -e POSTGRES_PASSWORD=123 -p 5432:5432 postgres

```

## Create database and table

```bash

docker exec -it some-postgres psql -U test

```

```sql

CREATE DATABASE rust_db;

\c rust_db


CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL
);

```

### APIs

1. Create User

```bash

curl -X POST -H "Content-Type: application/json" -d '{"name": "", "email": ""}' http://localhost:8080/create

```

2. Get Users

```bash

curl -X GET http://localhost:8080/get_all

```

3. Get User by ID

```bash

curl -X GET http://localhost:8080/get/:id

```

4. Update User

```bash

curl -X PUT -H "Content-Type: application/json" -d '{"name": "", "email": ""}' http://localhost:8080/update/:id

```

5. Delete User

```bash

curl -X DELETE http://localhost:8080/delete/:id

```


