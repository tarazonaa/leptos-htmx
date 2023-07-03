use leptos::*;

fn main() {
    mount_to_body(|cx| view! {cx, <App />})
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div id="tabs" hx-get="tab1.html" hx-trigger="load delay:100ms" hx-target="#tabs" hx-swap="innerHTML">
        </div>
    }
}
