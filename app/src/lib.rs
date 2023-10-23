use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    components::{footer::Footer, nav_bar::NavBar},
    pages::{counter::CounterPage, home::HomePage, todo::TodoPage, todos::TodosPage, Page},
};

mod api;
mod components;
pub mod error_template;
mod pages;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let routes = vec![
        Page {
            link: "/",
            name: "Home",
        },
        Page {
            link: "/counter",
            name: "Counter",
        },
        Page {
            link: "/todo",
            name: "Todos",
        },
    ];

    leptos::view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <NavBar pages=routes/>
            <main>
                <Routes>
                    <Route path="" view=HomePage />
                    <Route path="/counter" view=CounterPage />
                    <Route path="/todo" view=TodosPage />
                    <Route path="/todo/:id" view=TodoPage />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
