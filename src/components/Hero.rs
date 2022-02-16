use stylist::yew::{styled_component};
use yew::prelude::*;

#[styled_component(Hero)]
pub fn hero() -> Html {
    html! {
        <div class={css!(r#"
            display: flex;
            justify-content: center;
            align-items: center;
            flex-grow: 1;
        "#)}>
        { "Welcome to my website! It's a work in progress, currently learning to use Rust and Yew!" }
        </div>
    }
}

