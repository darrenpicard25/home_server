use std::sync::OnceLock;

use leptos::Serializable;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: String,
}

fn client() -> &'static Client {
    static CLIENT_INSTANCE: OnceLock<Client> = OnceLock::new();

    CLIENT_INSTANCE.get_or_init(|| Client::new())
}

pub async fn fetch_todos() -> Vec<Todo> {
    let json = client()
        .get("http://localhost:8000/api/todo")
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .unwrap()
        .text()
        .await
        .unwrap();

    log::info!("{json}");

    Vec::<Todo>::de(&json)
        .map_err(|e| log::error!("{e}"))
        .unwrap()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTodoPayload {
    pub title: String,
    pub description: String,
}

pub async fn create_todo(input: &CreateTodoPayload) -> Option<Todo> {
    let json = client()
        .post("http://localhost:8000/api/todo")
        .json(input)
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;

    Todo::de(&json).map_err(|e| log::error!("{e}")).ok()
}

pub async fn fetch_todo(id: i64) -> Todo {
    let url = format!("http://localhost:8000/api/todo/{id}");
    let json = client()
        .get(url)
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .unwrap()
        .text()
        .await
        .unwrap();

    log::info!("{json}");

    Todo::de(&json).map_err(|e| log::error!("{e}")).unwrap()
}
