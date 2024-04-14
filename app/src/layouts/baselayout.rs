use leptos::*;

#[component]
fn MainContainer(children: Children) -> impl IntoView {
    view! { <div class="grid gap-8 p-3">{children()}</div> }
}
#[component]
pub fn BaseLayout(children: Children) -> impl IntoView {
    view! {
        <div class="container mx-auto flex flex-col gap-5 overflow-y-auto">
            <MainContainer>{children()}</MainContainer>
        </div>
    }
}
