use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Deserialize, Clone, PartialEq)]
struct Post {
    id: u32,
    title: String,
}

#[derive(Clone, Routable)]

#[component]
fn PostList() -> Html {
    let posts = use_state(|| None::<Result<Vec<Post>, String>>);

    {
        let posts = posts.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let result = Request::get("https://jsonplaceholder.typicode.com/posts")
                    .send()
                    .await
                    .map_err(|e| e.to_string());

                let parsed = match result {
                    Ok(response) => response
                        .json::<Vec<Post>>()
                        .await
                        .map_err(|e| e.to_string()),
                    Err(e) => Err(e),
                };

                posts.set(Some(parsed));
            });
            || ()
        });
    }

    match &*posts {
        None => html! { <p>{ "Loading..." }</p> },
        Some(Ok(list)) => html! {
            <ul>
                { for list.iter().map(|p| html! { <li key={p.id}>{ &p.title }</li> }) }
            </ul>
        },
        Some(Err(err)) => html! { <p>{ format!("failed to load posts: {err}") }</p> },
    }
}



fn main() {
    yew::Renderer::<PostList>::new().render();
}