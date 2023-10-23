use leptos::*;
use leptos_router::*;

use crate::api::{self};

#[derive(Params, PartialEq)]
struct TodoParams {
    id: i64,
}

#[component]
pub fn TodoPage() -> impl IntoView {
    let id = || {
        use_params::<TodoParams>()
            .with(|params| params.as_ref().map(|params| params.id).unwrap_or_default())
    };
    let todo_resource =
        create_blocking_resource(id, |page_id| async move { api::fetch_todo(page_id).await });

    view! {
        <div class="max-w-4xl py-4 px-8 bg-white shadow-lg rounded-lg my-20 center m-auto">
            <Suspense fallback= || view! { "Loading..."}>
            {move || todo_resource.with(|optional_todo| {
                match optional_todo.clone() {
                    Some(todo) => {
                        view! {
                            <div>
                            <table>
                                <tr>
                                    <th>Id</th>
                                    <th>Title</th>
                                    <th>Description</th>
                                </tr>
                                <tr>
                                    <th>{todo.id}</th>
                                    <th>{todo.title}</th>
                                    <th>{todo.description}</th>
                                </tr>
                            </table>
                            </div>
                            }
                    },
                    None => view!{ <div>"Could not find todo "{id}</div>},
    }
            })}
            </Suspense>
        </div>
            }
}
