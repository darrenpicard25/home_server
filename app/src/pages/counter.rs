use leptos::*;
use web_sys::MouseEvent;

use crate::components::button::Button;

#[component]
pub fn CounterPage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = Box::new(move |_: MouseEvent| set_count.update(|count| *count += 1));

    view! {
    <div class="max-w-md py-4 px-8 bg-white shadow-lg rounded-lg my-20 center m-auto">
      <div>
        <h2 class="text-gray-800 text-3xl font-semibold">"Smokey Olive Counter"</h2>
        <p class="mt-2 text-gray-600">"Current Count: "{count}</p>
        <Button on_click=Some(on_click)>"Click"</Button>
      </div>
    </div>
        }
}
