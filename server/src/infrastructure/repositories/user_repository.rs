use axum::async_trait;
use sqlx::{
    types::time::{OffsetDateTime, PrimitiveDateTime},
    Error,
};

use crate::{
    domain::{
        entities::user::User,
        repositories::{
            error::{RepositoryError, RepositoryResult},
            user_repository::{CreateInput, UpdateInput, UserRepositoryPort},
        },
    },
    infrastructure::Database,
};

#[derive(sqlx::FromRow, Debug)]
struct UserDocument {
    id: i64,
    email: String,
    first_name: String,
    #[allow(dead_code)]
    created_at: PrimitiveDateTime,
    #[allow(dead_code)]
    updated_at: PrimitiveDateTime,
}

impl From<UserDocument> for User {
    fn from(val: UserDocument) -> Self {
        User {
            id: val.id,
            email: val.email,
            first_name: val.first_name,
        }
    }
}

pub struct UserRepository {
    db: Database,
}

impl UserRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepositoryPort for UserRepository {
    async fn find_by_id(&self, id: i64) -> RepositoryResult<User> {
        log::debug!("UserRepository.find_by_id | {id}");

        let document = sqlx::query_as::<_, UserDocument>("SELECT * FROM users WHERE id = $1")
            .bind(&id)
            .fetch_one(&self.db.pool())
            .await
            .map_err(|e| {
                log::error!("{e}");
                match e {
                    Error::RowNotFound => RepositoryError::NotFound,
                    e => RepositoryError::Unknown(e.to_string()),
                }
            })?;

        Ok(document.into())
    }

    async fn find_by_email(&self, email: String) -> RepositoryResult<Option<User>> {
        log::debug!("UserRepository.find_by_email | {email}");

        let document = sqlx::query_as::<_, UserDocument>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(&self.db.pool())
            .await;

        match document {
            Ok(doc) => Ok(Some(doc.into())),
            Err(Error::RowNotFound) => Ok(None),
            Err(e) => Err(RepositoryError::Unknown(e.to_string())),
        }
    }

    async fn update_one(&self, id: i64, input: UpdateInput) -> RepositoryResult<User> {
        log::debug!("UserRepository.update_one | {id} | {input:?}");

        let user = self.find_by_id(id).await?;
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());

        let document = sqlx::query_as::<_, UserDocument>(
            r#"UPDATE users
            SET
            email = $1,
            first_name = $2,
            updated_at = $3
            WHERE id = $4
            RETURNING *"#,
        )
        .bind(input.email)
        .bind(input.first_name)
        .bind(now)
        .bind(user.id)
        .fetch_one(&self.db.pool())
        .await
        .map_err(|e| {
            log::error!("{e}");
            match e {
                Error::RowNotFound => RepositoryError::NotFound,
                e => RepositoryError::Unknown(e.to_string()),
            }
        })?;

        Ok(document.into())
    }

    async fn create(&self, input: CreateInput) -> RepositoryResult<User> {
        log::debug!("UserRepository.create | {input:?}");

        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());

        let document = sqlx::query_as::<_, UserDocument>(
            r#"INSERT INTO users
            (email, first_name, created_at, updated_at)
            VALUES ($1, $2, $3, $4)
            RETURNING *"#,
        )
        .bind(input.email)
        .bind(input.first_name)
        .bind(now)
        .bind(now)
        .fetch_one(&self.db.pool())
        .await
        .map_err(|e| RepositoryError::Unknown(e.to_string()))?;

        Ok(document.into())
    }
}
