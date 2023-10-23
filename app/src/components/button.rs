use leptos::*;
use web_sys::MouseEvent;

const BASE_BUTTON_CLASS: &'static str = "middle none center m-4 rounded-lg bg-blue-500 py-3 px-6 font-bold uppercase text-white shadow-md shadow-blue-500/20 transition-all hover:shadow-lg hover:shadow-blue-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none";

type OnClickFunction = Option<Box<dyn Fn(MouseEvent) + 'static>>;

#[component]
pub fn Button(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] on_click: OnClickFunction,
    #[prop(default = "button")] button_type: &'static str,
) -> impl IntoView {
    view! {
        <button
            class={class.unwrap_or(BASE_BUTTON_CLASS)}
            data-ripple-light="true"
            type=button_type
            on:click=on_click.unwrap_or_else(|| Box::new(|_| {}))
            >{children.map(|child| child())}
        </button>
    }
}
