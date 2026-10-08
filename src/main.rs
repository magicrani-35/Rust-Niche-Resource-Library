use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <section>
                <h1>{ "Explore the museum" }</h1>
                <p>{ "Discover artwork and collect your favorites." }</p>
            </section>
        },
        Route::About => html! {
            <section>
                <h1>
                    { "About this museum" }
                </h1>
                <p>
                    { "A small art discovery app built with Rust and Yew." }
                </p>
            </section>
        },
        Route::NotFound => html! {
            <section>
                <h1>{ "Page not found" }</h1>
                <Link<Route> to={Route::Home}>
                    { "Return to the gallery" }
                </Link<Route>>
            </section>
        },
    }
}

#[component]
fn App() -> Html {
      html! {
          <BrowserRouter>
            <header>
                <p>
                    { "The Little Museum" }
                </p>
                <nav>
                    <Link<Route> to={Route::Home}>
                        { "Gallery" }
                    </Link<Route>>

                    { " | " }

                    <Link<Route> to={Route::About}>
                        { "About" }
                    </Link<Route>>
                </nav>
            </header>

            <main>
                <Switch<Route> render={switch} />
            </main>
          </BrowserRouter>
      }
}

fn main() {
    yew::Renderer::<App>::new().render();
}