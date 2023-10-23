use leptos::html::Input;
use leptos::*;

const BASE_INPUT_CLASS: &'static str = "shadow appearance-none border border-red-500 rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline";

#[component]
pub fn Input(
    id: &'static str,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] input_ref: Option<NodeRef<Input>>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let input_ref = input_ref.unwrap_or_else(|| create_node_ref());
    view! {
        <div class="mb-6">
        <label class="block text-gray-700 text-sm font-bold mb-2" for={id}>
            {label}
        </label>
        <input class={class.unwrap_or(BASE_INPUT_CLASS)} ref={input_ref} id={id} type="text" placeholder={placeholder}/>
    </div>
    }
}
