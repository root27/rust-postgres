use serde:: {Serialize, Deserialize};

use sqlx::FromRow;



#[derive(Serialize, FromRow, Deserialize)]

pub struct User {
    pub id :Option<i32>,
    pub name: String,
    pub email: String
}



impl User {
    pub async fn create(user: User, db: &sqlx::Pool<sqlx::Postgres>) -> Result<User, sqlx::Error> {
        let row = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (name, email)
            VALUES ($1, $2)
            RETURNING id, name, email
            "#,
        )
        .bind(user.name)
        .bind(user.email)
        .fetch_one(db)
        .await?;
        Ok(row)
    }

    pub async fn get_all(db: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<User>, sqlx::Error> {
        let rows = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, email
            FROM users
            "#,
        )
        .fetch_all(db)
        .await?;
        Ok(rows)
    }

    pub async fn get(id: i32, db: &sqlx::Pool<sqlx::Postgres>) -> Result<User, sqlx::Error> {
        let row = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, email
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(db)
        .await?;
        Ok(row)
    }

    pub async fn update(id: i32, user: User, db: &sqlx::Pool<sqlx::Postgres>) -> Result<User, sqlx::Error> {
        let row = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET name = $1, email = $2
            WHERE id = $3
            RETURNING id, name, email
            "#,
        )
        .bind(user.name)
        .bind(user.email)
        .bind(id)
        .fetch_one(db)
        .await?;
        Ok(row)
    }

    pub async fn delete(id: i32, db: &sqlx::Pool<sqlx::Postgres>) -> Result<User, sqlx::Error> {
        let row = sqlx::query_as::<_, User>(
            r#"
            DELETE FROM users
            WHERE id = $1
            RETURNING id, name, email
            "#,
        )
        .bind(id)
        .fetch_one(db)

        .await?;

        Ok(row)
    }
}

