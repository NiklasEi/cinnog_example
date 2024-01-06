use leptos::*;

#[island]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <div class="center">
            <button on:click=on_click>"Click Me: " {count}</button>
        </div>
    }
}
