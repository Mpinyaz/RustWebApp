pub mod components;
pub mod error_template;

use crate::components::footer::HomeFooter;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_app.css"/>
        <Router fallback=| |{
        let mut outside_errors = Errors::default();
        outside_errors.insert_with_default_key(AppError::NotFound);
        view! {
            <ErrorTemplate outside_errors/>
        }
            .into_view()
    }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <Title text="Leptos + Tailwindcss"/>
        <h1 class="text-4xl text-center p-5">Hello from Leptos + Tailwindcss</h1>
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button on:click=on_click class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "Click number " {count}
                    </button>
                </div>
            </div>
        <HomeFooter/>
        </main>
    }
}
