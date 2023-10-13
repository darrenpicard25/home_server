use axum::async_trait;
use sqlx::{
    types::time::{OffsetDateTime, PrimitiveDateTime},
    Error, FromRow,
};

use crate::{
    domain::{
        entities::todo::Todo,
        repositories::{
            error::{RepositoryError, RepositoryResult},
            todo_repository::{CreateInput, TodoRepositoryPort, UpdateInput},
        },
    },
    infrastructure::Database,
};

#[derive(FromRow, Debug)]
struct TodoDocument {
    id: i64,
    title: String,
    description: String,
    #[allow(dead_code)]
    created_at: PrimitiveDateTime,
    #[allow(dead_code)]
    updated_at: PrimitiveDateTime,
}

impl From<TodoDocument> for Todo {
    fn from(val: TodoDocument) -> Self {
        Todo {
            id: val.id,
            title: val.title,
            description: val.description,
        }
    }
}

pub struct TodoRepository {
    db: Database,
}

impl TodoRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TodoRepositoryPort for TodoRepository {
    async fn list(&self) -> RepositoryResult<Vec<Todo>> {
        tracing::debug!("TodoRepository.list");

        let documents = sqlx::query_as::<_, TodoDocument>("SELECT * FROM todos")
            .fetch_all(&self.db.pool())
            .await
            .map_err(|e| RepositoryError::Unknown(e.to_string()))?;

        Ok(documents.into_iter().map(|doc| doc.into()).collect())
    }

    async fn find_by_id(&self, id: i64) -> RepositoryResult<Todo> {
        tracing::debug!("TodoRepository.find_by_id | {id}");

        let document = sqlx::query_as::<_, TodoDocument>("SELECT * FROM todos WHERE id = $1")
            .bind(&id)
            .fetch_one(&self.db.pool())
            .await
            .map_err(|e| {
                tracing::error!("{e}");
                match e {
                    Error::RowNotFound => RepositoryError::NotFound,
                    e => RepositoryError::Unknown(e.to_string()),
                }
            })?;

        Ok(document.into())
    }

    async fn update_one(&self, input: UpdateInput) -> RepositoryResult<Todo> {
        tracing::debug!("TodoRepository.update_one | {input:?}");

        let document = self.find_by_id(input.id).await?;
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());

        let document = sqlx::query_as::<_, TodoDocument>(
            r#"UPDATE todos
            SET
            title = $1,
            description = $2,
            updated_at = $3
            WHERE id = $4
            RETURNING *"#,
        )
        .bind(input.title.unwrap_or(document.title))
        .bind(input.description.unwrap_or(document.description))
        .bind(now)
        .bind(&document.id)
        .fetch_one(&self.db.pool())
        .await
        .map_err(|e| {
            tracing::error!("{e}");
            match e {
                Error::RowNotFound => RepositoryError::NotFound,
                e => RepositoryError::Unknown(e.to_string()),
            }
        })?;

        Ok(document.into())
    }

    async fn create(&self, input: CreateInput) -> RepositoryResult<Todo> {
        tracing::debug!("TodoRepository.create | {input:?}");

        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());

        let document = sqlx::query_as::<_, TodoDocument>(
            r#"INSERT INTO todos
            (title, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4)
            RETURNING *"#,
        )
        .bind(input.title)
        .bind(input.description)
        .bind(now)
        .bind(now)
        .fetch_one(&self.db.pool())
        .await
        .map_err(|e| RepositoryError::Unknown(e.to_string()))?;

        Ok(document.into())
    }
}
