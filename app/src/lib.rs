use leptos::*;
use leptos_meta::*;

use crate::components::default_button::DefaultButton;

mod components;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    let (count, set_count) = create_signal(cx, 0);

    let on_click = move |_| {
        set_count.update(|current_value| {
            *current_value += 1;
        })
    };

    view! {
        cx,
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/member_portal.css"/>
        <Title text="Cargo Leptos" />
        <div class="grid grid-cols-6 grid-rows-4">
            <div class="col-span-6 row-span-1 bg-color4">
                <h1 class="mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white">"Smokey Olive"</h1>
            </div>
            <div class="grid grid-cols-3 row-span-1 col-span-5">
                <h1 class="col-span-1">"Hi from your Leptos WASM! With live reload"</h1>
                <div class="col-span-1">
                    <DefaultButton on_click text="Click Me"/>
                </div>
                <h3 class="col-span-1">{count}</h3>
            </div>
            <div class="col-span-1 row-span-1" >
                <h3>"Nav Bar"</h3>
            </div>
            <div class="col-span-6 row-span-1 bg-color1" >
                <h3>"Footer"</h3>
            </div>
        </div>
    }
}
