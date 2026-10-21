mod api;
mod app;
mod models;
mod routes;
mod components;
mod pages;
mod favorites;

use crate::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
