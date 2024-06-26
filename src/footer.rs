use yew::prelude::*;

#[function_component(FooterComp)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="container text-center">
            {"Visit "}<a href="https://konnektoren.help">{"Konnektoren.help"}</a>
            </div>
        </footer>
    }
}
