mod api;
mod app;
mod models;
mod routes;
mod components;
mod pages;

use crate::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
