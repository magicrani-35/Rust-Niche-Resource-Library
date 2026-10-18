use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub(crate) struct PaginationControlsProps {
    pub(crate) current_page: u32,
    pub(crate) last_page: u32,
    pub(crate) on_page_change: Callback<u32>,
}

#[component]
pub(crate) fn PaginationControls(
    props: &PaginationControlsProps,
) -> Html {
     let previous_page = {
         let on_page_change = props.on_page_change.clone();
         let current_page = props.current_page;

         Callback::from(move |_| {
             on_page_change.emit(
                 current_page.saturating_sub(1).max(1)
             );
         })
     };

    let next_page = {
        let on_page_change = props.on_page_change.clone();
        let current_page = props.current_page;
        let last_page = props.last_page;

        Callback::from(move |_| {
            on_page_change.emit(
                current_page.saturating_add(1).min(last_page)
            );
        })
    };

    html! {
        <nav class="pagination" aria-label="Artwork result pages">
            <button
                type="button"
                onclick={previous_page}
                disabled={props.current_page <= 1}
            >
                 { "← Previous" }
            </button>

            <span aria-live="polite">
                { format!("Page {}", props.current_page) }
            </span>

            <button
                type="button"
                onclick={next_page}
                disabled={props.current_page >= props.last_page}
            >
                { "Next →" }
            </button>
        </nav>
    }
}