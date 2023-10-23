use leptos::{html::Input, *};
use leptos_router::*;
use web_sys::SubmitEvent;

use crate::{
    api::{self, create_todo, CreateTodoPayload},
    components::button::Button,
    components::input::Input as CustomInput,
};

#[component]
pub fn TodosPage() -> impl IntoView {
    let todos_resource = create_blocking_resource(|| (), |_| async { api::fetch_todos().await });
    let input_title: NodeRef<Input> = create_node_ref();
    let input_description: NodeRef<Input> = create_node_ref();

    let handle_todo_create = create_action(move |input: &CreateTodoPayload| {
        let input = input.clone();
        async move {
            let results = create_todo(&input).await;

            let Some(todo) = results else {
                log::error!("Something went wrong");
                return;
            };

            todos_resource.update(|todos| {
                if let Some(todos) = todos {
                    todos.push(todo);
                }
            });
        }
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let title_ref = input_title().expect("title input to exist");
        let description_ref = input_description().expect("description input to exist");

        let title = title_ref.value();
        title_ref.set_value("");

        let description = description_ref.value();
        description_ref.set_value("");

        handle_todo_create.dispatch(CreateTodoPayload { title, description });
    };

    view! {
        <div class="max-w-4xl py-4 px-8 bg-white shadow-lg rounded-lg my-20 center m-auto">
            <Suspense fallback= || view! { "Loading..."}>
            {move || todos_resource.with(|optional_todos| {
                match optional_todos.clone() {
                    Some(todos) if todos.len() == 0 => view! { <div>"No Todos"</div>},
                    Some(todos) => {
                        view! {
                            <div>
                            <table>
                                <tr>
                                    <th>Id</th>
                                    <th>Title</th>
                                    <th>Description</th>
                                </tr>
                                <For
                                    each=move || todos.clone()
                                    key=|todo| todo.id
                                    let:todo
                                >
                                    <tr>
                                        <td><A href={todo.id.to_string()}>{todo.id}</A></td>
                                        <td>{todo.title}</td>
                                        <td>{todo.description}</td>
                                    </tr>
                                </For>
                            </table>
                            </div>
                            }
                    },
                    None => view!{ <div>"No Todos"</div>},
    }
            })}
            </Suspense>
        </div>
        <div class="max-w-2xl py-4 px-8 bg-white shadow-lg rounded-lg my-20 center m-auto">
        <form on:submit=on_submit >
            <CustomInput id="title" label="Title" input_ref={input_title} placeholder="Title"/>
            <CustomInput id="description" label="Description" input_ref={input_description} placeholder="Description"/>
            <div class="flex items-center justify-between">
                <Button button_type="submit">"Sign In"</Button>
            </div>
        </form>
    </div>
            }
}
