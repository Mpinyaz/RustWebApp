use leptos::*;
use leptos_dom::*;
use leptos_router::*;

#[derive(Debug, Clone)]
pub struct NavItem {
    pub path: String,
    pub name: String,
    pub active: bool,
}

impl NavItem {
    pub fn new(path: &str, name: &str, active: bool) -> Self {
        Self {
            path: path.to_string(),
            name: name.to_string(),
            active,
        }
    }

    pub fn href(&self) -> String {
        if !self.path.starts_with('/') {
            let mut href = self.path.clone();
            href.insert(0, '/');
            href
        } else {
            self.path.clone()
        }
    }
}

impl IntoView for NavItem {
    fn into_view(self) -> View {
        if self.active {
            view! {<li class="py-2 text-zinc-400 underline underline-offset-4">{self.name}</li>}
                .into_view()
        } else {
            view! {<li class="py-2 text-zinc-200 hover:text-zinc-400 underline underline-offset-4">
            <A href=self.href()>{self.name}</A></li>}
            .into_view()
        }
        .into_view()
    }
}
